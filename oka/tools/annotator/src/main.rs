#![allow(non_snake_case, unused)]
// Copyright 2020 Google LLC
// Copyright 2020 Team Spacecat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate clap;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate tempfile;
#[macro_use]
extern crate itertools;

use std::io::Read;

type Error = Box<dyn std::error::Error>;

fn main() {
    run().unwrap_or_else(|s| {
        eprintln!("{}", s);
        std::process::exit(1);
    });
}

fn writer(file: Option<&str>) -> Result<Box<dyn std::io::Write>, Error> {
    Ok(if let Some(f) = file {
        Box::new(std::fs::File::create(f)?)
    } else {
        Box::new(std::io::stdout())
    })
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Lab {
    Pegovka,     // numbers, variables, operators, ...
    Pflockingen, // molecules, lives, ...
}

static mut LABORATORY: Lab = Lab::Pegovka;

fn laboratory() -> Lab {
    unsafe { LABORATORY }
}

fn run() -> Result<(), Error> {
    use clap::{App, Arg, SubCommand};

    let matches = App::new("oka")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("output file name")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("png2txt")
                .about("parse given png image to txt format and output")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .index(1),
                ),
            SubCommand::with_name("annotate")
                .about("annotate the given program file (png or txt) and output")
                .arg(
                    Arg::with_name("pflockingen")
                        .short("f")
                        .long("pflockingen")
                        .help(
                            "Annotates messages from the Pflockingen laboratory (default is pegovka)",
                        ),
                )
                .arg(
                    Arg::with_name("text")
                        .short("t")
                        .long("text")
                        .help(
                            "Output in text format instead of svg",
                        ),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .index(1),
                ),
        ])
        .get_matches();

    let mut w = writer(matches.value_of("output"))?;

    match matches.subcommand() {
        ("png2txt", Some(m)) => {
            let grid = parse_file(m.value_of("INPUT"))?;
            output(&mut w, &grid)?;
        }
        ("annotate", Some(m)) => {
            if m.is_present("pflockingen") {
                unsafe {
                    LABORATORY = Lab::Pflockingen;
                }
            }
            let in_text = m.is_present("text");

            let grid = parse_file(m.value_of("INPUT"))?;
            let glyphs = parse(&grid)?;

            if in_text {
                output_txt(&mut w, &glyphs);
            } else {
                output_svg(&mut w, &grid, &glyphs)?;
            }
        }
        _ => return Err("No such subcommand".into()),
    };
    Ok(())
}

fn output_txt(w: &mut impl std::io::Write, glyphs: &Vec<Glyph>) {
    let mut s = glyphs.iter().map(|g| g.rows.start).collect::<Vec<_>>();
    s.sort();
    s.dedup();
    // TODO: scale.
    for r in s {
        let mut gs: Vec<_> = glyphs.iter().filter(|g| g.rows.start == r).collect();
        gs.sort_by_key(|g| g.cols.start);
        for g in gs {
            print!("{} ", g.k.to_string());
        }
        println!();
    }
}

fn output_svg(
    w: &mut impl std::io::Write,
    grid: &Grid,
    glyphs: &Vec<Glyph>,
) -> Result<(), std::io::Error> {
    const SZ: usize = 14;
    let H = grid.len();
    let W = grid[0].len();
    let out = format!(
        r##"<?xml version="1.0"?>
<svg xmlns="http://www.w3.org/2000/svg" height="{}" width="{}">
  <rect y="0" x="0" height="{0}" width="{1}" fill="black" />
  {}
  {}
</svg>"##,
        H * SZ,
        W * SZ,
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, c)| {
                        if *c {
                            Some(format!(
                                r##"<rect y="{}" x="{}" height="{}" width="{}" fill="#757575"/>
    "##,
                                i * SZ,
                                j * SZ,
                                SZ,
                                SZ
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>()
            .join(""),
        glyphs.iter().map(|g| {
            let (x1,x2) = (g.rows.start * SZ, g.rows.end * SZ);
            let (y1,y2) = (g.cols.start * SZ, g.cols.end * SZ);
            let color = match g.k {
                Kind::Int(_) | Kind::Var(_) | Kind::Binary(_) => "#004D40", // green
                Kind::Molecule(_) | Kind::Amino(_,_) => "#01579B", // blue
                Kind::Unnamed(_) => "#BF360C", // red
                _ => "#827717", // yellow
            };
            format!(
                r#"<rect y="{}" x="{}" height="{}" width="{}" fill="{}" fill-opacity="0.5"/>
<text y="{}" x="{}" dominant-baseline='middle' text-anchor='middle' fill='white' style='paint-order: stroke; fill: white; stroke: black; stroke-width: 1px; font-family="monospace"; font-size: 18px'>{}</text>
"#,
            x1,
            y1,
            (x2-x1),
            (y2-y1),
            color,
            (x1+x2)/2,
            (y1+y2)/2,
            {
                let n = g.k.to_string().split("\n").count() as isize;
                g.k.to_string().split("\n").into_iter().enumerate().map(|(i,s)|format!(r#"<tspan x="{}" dy="{}em">{}</tspan>"#, (y1+y2)/2, if i==0 {-(n/2) as f64} else {1.1}, s)).collect::<Vec<String>>().join("\n")
            },
            )
        }).collect::<Vec<String>>().join("")
    );
    writeln!(w, "{}", out)
}

type Grid = Vec<Vec<bool>>;

#[derive(Debug)]
struct Glyph {
    rows: std::ops::Range<usize>,
    cols: std::ops::Range<usize>,
    k: Kind,
}

#[derive(Clone, Debug)]
enum Kind {
    Int(isize),
    Is,
    Apply,
    Succ,
    Pred,
    Sum,
    Var(usize),
    Product,
    Quotient, // integer division
    Equals,
    Bool(bool),
    LT, // less than
    ToBin,
    FromBin,
    Binary(isize),
    Op(&'static str),
    Pow(usize), // x^?
    LBra,
    RBra,
    Separator,

    Molecule(&'static str),
    Amino(&'static str, &'static str),
    Life(&'static str),
    Unnamed(String),
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Kind::Int(n) => format!(
                "{}{}",
                n,
                if laboratory() == Lab::Pflockingen
                    && (1 <= *n && (*n as usize) - 1 < ELEMENTS.len())
                {
                    format!("\n{}", ELEMENTS[*n as usize - 1])
                } else {
                    "".into()
                }
            ),
            Kind::Is => "is".into(),
            Kind::Apply => "ap".into(),
            Kind::Succ => "Succ".into(),
            Kind::Pred => "Pred".into(),
            Kind::Sum => "Sum".into(),
            Kind::Var(i) => format!("x{}", i),
            Kind::Product => "Prod".into(),
            Kind::Quotient => "Div".into(),
            Kind::Equals => "==".into(),
            Kind::Bool(true) => "True".into(),
            Kind::Bool(false) => "False".into(),
            Kind::LT => "LT".into(),
            Kind::ToBin => "ToBin".into(),
            Kind::FromBin => "FromBin".into(),
            Kind::Binary(i) => format!("Bin({})", i),
            Kind::Op(ref s) => s.to_string(),
            Kind::Pow(base) => format!("{}^", base),
            Kind::LBra => "[".into(),
            Kind::RBra => "]".into(),
            Kind::Separator => ";".into(),

            Kind::Molecule(s) => format!(
                "{}{}",
                s.to_string(),
                NUM_TO_KIND
                    .iter()
                    .find(|(i, k)| if let Kind::Molecule(t) = k {
                        t == s
                    } else {
                        false
                    })
                    .and_then(|(i, _)| Some(format!("\n({})", i)))
                    .unwrap_or("".to_string()),
            ),
            Kind::Amino(_, abbr) => format!("{}", abbr),
            Kind::Life(s) => s.to_string(),
            Kind::Unnamed(x) => format!("{}?", x),
        }
    }
}

lazy_static! {
    static ref ELEMENTS: Vec<&'static str> = vec![
        "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S",
        "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga",
        "Ge", "As", "Se", "Br"
    ];
    static ref NUM_TO_KIND: std::collections::HashMap<isize, Kind> = {
        let mut m = std::collections::HashMap::new();
        m.insert(0, Kind::Apply);
        m.insert(12, Kind::Is);
        m.insert(365, Kind::Sum);
        m.insert(401, Kind::Pred);
        m.insert(417, Kind::Succ);
        m.insert(146, Kind::Product);
        m.insert(40, Kind::Quotient);
        m.insert(448, Kind::Equals);
        m.insert(2, Kind::Bool(true));
        m.insert(8, Kind::Bool(false));
        m.insert(416, Kind::LT);
        m.insert(170, Kind::ToBin);
        m.insert(341, Kind::FromBin);
        m.insert(174, Kind::Unnamed("op15".into()));
        m.insert(10, Kind::Op("Neg")); // 16
        m.insert(7, Kind::Op(r#"\f g x -> (f x) (g x)"#)); // 18
        m.insert(6, Kind::Op(r#"\f x y -> f y x"#)); // 19
        m.insert(5, Kind::Op(r#"\x y z -> x (y z)"#)); // 20
        m.insert(1, Kind::Op(r#"\x -> x"#)); // 24
        m.insert(14, Kind::Op(r#"nil"#)); // 28
        m.insert(15, Kind::Op(r#"isnil"#)); // 29
        m.insert(17043521, Kind::Op(r#"arrow"#)); // 31
        m.insert(33047056, Kind::Op(r#"set_pixel"#)); // 32
        // m.insert(11184810, Kind::Op(r#""#)); // 33
        m.insert(11184810, Kind::Op(r#"map_set_pixel"#)); // 34
        m.insert(58336, Kind::Op(r#"if0"#)); // 37
        m.insert(33053392, Kind::Unnamed(r#"op38"#.to_string())); // 38

        m.insert(16, Kind::Molecule("CH4"));
        m.insert(17, Kind::Molecule("NH3"));
        m.insert(18, Kind::Molecule("H2O"));
        m.insert(28, Kind::Molecule("N2"));
        m.insert(32, Kind::Molecule("O2"));
        m.insert(46, Kind::Molecule("SiO2"));
        m.insert(48, Kind::Molecule("O3"));
        m.insert(328, Kind::Molecule("FeO"));
        m.insert(74, Kind::Molecule("Al2O3"));
        m.insert(85, Kind::Molecule("NaNo3"));
        m.insert(95, Kind::Molecule("MgCl2"));
        m.insert(104, Kind::Molecule("Fe2O3"));

        m.insert(56, Kind::Amino("-NH-C(R)H-C(=O)-", "Peptide"));
        m.insert(20, Kind::Amino("-CH(NH2)-CH3", "Bal"));
        m.insert(29, Kind::Amino("-CH2-\nCH3", "Mal"));
        m.insert(30, Kind::Amino("-CH2-\nNH2", "Aal"));
        m.insert(31, Kind::Amino("-CH2-NH2", "Ser"));
        m.insert(44, Kind::Amino("-CH(OH)-CH3", "Thr"));
        m.insert(45, Kind::Amino("-CH2-\nCH2-\nOH", "Msr"));
        m.insert(58, Kind::Amino("-CH2-C(=O)-NH2", "Asn"));

        m.insert(59, Kind::Amino("-CH2-\nC(=O)-\nOH", "Asp"));
        m.insert(72, Kind::Amino("-C2H4-\nC(=O)-\nNH2", "Gln"));
        m.insert(9, Kind::Amino("-C2H4-\nC(=O)-\nOH", "Glu"));
        m.insert(84, Kind::Amino("-CH2-\nCH(-CH3)2", "Leu"));
        m.insert(86, Kind::Amino("-CH2-\nCH2-NH-\nC(NH2)(NH)", "Arg"));
        m.insert(87, Kind::Amino("-CH2-\nCH2-CH2-\nCOOH", "Mgu"));
        m.insert(100, Kind::Amino("-CH2-CH2-\nCH2-NH-\nC(NH2)(NH)", "Mar"));
        m.insert(118, Kind::Amino("-CH2-\nC6H5", "Phe"));
        m.insert(134, Kind::Amino("-CH2-\nC6H4-OH", "Tyr"));
        m
    };
    static ref FIXED: Vec<(Vec<&'static str>, Kind)> = vec![
        (vec!["1000", "0100", "1100", "0011"], Kind::Molecule("D")),
        (vec!["1000", "0111", "1100", "0011"], Kind::Molecule("T")),
        (
            vec!["1010", "0101", "1101", "1000"],
            Kind::Molecule("amino")
        ),
        (vec!["101", "010", "101"], Kind::Molecule("bond")),
        (vec!["1000", "0111", "0111", "0010"], Kind::Life("Ser life")),
        (
            vec!["1001", "0111", "0010", "0010"],
            Kind::Molecule("Ser life\nbeta sheet")
        ),
        (
            vec!["1000", "0111", "0010", "0111"],
            Kind::Molecule("Ser life\nDNA")
        ),
        (vec!["1000", "0111", "0111", "0101"], Kind::Life("Asp life")),
        (
            vec!["1001", "0111", "0101", "0101"],
            Kind::Molecule("Asp life\nbeta sheet")
        ),
        (
            vec!["1000", "0111", "0101", "0111"],
            Kind::Molecule("Asp life\nDNA")
        ),
        (
            vec!["1010", "0101", "1101", "0010"],
            Kind::Molecule("Ser life\nR")
        ),
        (
            vec!["1010", "0101", "1101", "0101"],
            Kind::Molecule("Asp life\nR")
        ),
        (
            vec!["1011", "0111", "1111", "0010"],
            Kind::Molecule("Ser life\nprotein")
        ),
        (
            vec!["1011", "0111", "1111", "0101"],
            Kind::Molecule("Asp life\nprotein")
        ),
        (
            vec!["1000", "0111", "0111", "0111"],
            Kind::Life("Mother\nplanet")
        ),
        (
            vec!["1011", "0111", "1111", "1111"],
            Kind::Unnamed("13-1".into())
        ),
        (
            vec!["1010", "0111", "0101", "0111"],
            Kind::Unnamed("14-1".into())
        ),

        (
            vec!["11111", "10101", "10101", "10101", "11111"],
            Kind::Op("cos".into())
        ),
        (
            vec!["11111", "10111", "10101", "10101", "11111"],
            Kind::Op("car".into())
        ),
        (
            vec!["11111", "11101", "10101", "10101", "11111"],
            Kind::Op("cdr".into())
        ),
        (
            vec!["001", "011", "111", "011", "001"],
            Kind::LBra, // list
        ),
        (
            vec!["100", "110", "111", "110", "100"],
            Kind::RBra, // list
        ),
        (
            vec!["11", "11", "11", "11", "11"],
            Kind::Separator, // list
        ),
    ];
}

fn clip(g: &Grid, (x0, y0): (usize, usize), (x1, y1): (usize, usize)) -> Grid {
    g[x0..x1].iter().map(|row| row[y0..y1].to_vec()).collect()
}

fn parse_glyph(comp: &Grid, flip: bool) -> Option<Kind> {
    let inside = |i: usize, j: usize| i < comp.len() && j < comp[0].len();
    let get = |i: usize, j: usize| inside(i, j) && (comp[i][j] ^ flip);

    for (fig, k) in FIXED.iter() {
        let mut ok = true;
        if !(fig.len() == comp.len() && fig[0].len() == comp[0].len()) {
            continue;
        }
        if fig.iter().enumerate().all(|(i, row)| {
            row.chars()
                .enumerate()
                .all(|(j, c)| (c == '1') == get(i, j))
        }) {
            return Some(k.clone());
        }
    }

    if !flip && comp.len() == 2 {
        let bin = (|| {
            let n = comp[0].len();
            let bs = comp[0]
                .iter()
                .zip(comp[1].iter())
                .map(|(x, y)| if x == y { None } else { Some(*x) })
                .collect::<Option<Vec<_>>>()?;
            let negative = if bs.len() < 2 || bs[0] == bs[1] {
                None
            } else {
                Some(bs[0])
            }?;
            let k = (2..n).find(|i| !bs[*i])?;
            let m = (k - 2) * 4;
            if n - 1 != k + m {
                return None;
            }
            let num = bs.as_slice()[n - m..n]
                .iter()
                .fold(0isize, |acc, b| acc * 2 + (if *b { 1 } else { 0 }));
            Some(Kind::Binary(num * (if negative { -1 } else { 1 })))
        })();
        if bin.is_some() {
            return bin;
        }
    }

    // num or var
    if !((1..comp.len()).all(|i| get(i, 0)) && (1..comp[0].len()).all(|j| get(0, j))) {
        return None;
    }
    let n = comp[0].len() - 1;
    if n == 0 {
        return None;
    }
    // FIXME ?
    if n > 8 {
        return None;
    }

    if !(1..=n).all(|i| get(i, 0) && get(0, i)) {
        return None;
    }

    let is_var = n > 2 && (0..=n).all(|i| get(i, 0) && get(i, n) && get(0, i) && get(n, i));
    if is_var {
        match parse_glyph(&clip(comp, (1, 1), (n, n)), !flip) {
            Some(Kind::Int(i)) if i >= 0 => return Some(Kind::Var(i as usize)),
            _ => (),
        };
        if n > 5 {
            match parse_glyph(&clip(comp, (2, 2), (n - 1, n - 1)), flip) {
                Some(Kind::Int(i)) => return Some(Kind::Pow(i as usize)),
                _ => (),
            };
        }
    }

    let negative = get(n + 1, 0);

    let mut num = 0isize;
    let mut idx = 0usize;
    for i in 1..=n {
        for j in 1..=n {
            if get(i, j) {
                num += 1 << idx;
            }
            idx += 1;
        }
    }
    if negative {
        num = -num;
    }

    if !get(0, 0) {
        if (laboratory() == Lab::Pflockingen) && (num == 0) {
            return None;
        }
        return Some(Kind::Int(num));
    }

    Some(
        NUM_TO_KIND
            .get(&num)
            .map_or(Kind::Unnamed(format!("{}", num)), Clone::clone),
    )
}

const DX: [isize; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const DY: [isize; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

fn component(grid: &Grid, x: usize, y: usize, used: &mut Grid, cells: &mut Vec<(usize, usize)>) {
    if *used.get(x).and_then(|row| row.get(y)).unwrap_or(&true) || !grid[x][y] {
        return;
    }
    used[x][y] = true;
    cells.push((x, y));
    for (dx, dy) in DX.iter().zip(DY.iter()) {
        component(
            grid,
            (x as isize + dx) as usize,
            (y as isize + dy) as usize,
            used,
            cells,
        );
    }
}

fn parse(grid: &Grid) -> Result<Vec<Glyph>, Error> {
    let (h, w) = (grid.len(), grid[0].len());
    let mut used = vec![vec![false; w]; h];
    let mut res = vec![];
    for (x, y) in iproduct!(1..h - 1, 1..w - 1) {
        if used[x][y] || !grid[x][y] {
            continue;
        }

        let mut cells = vec![];
        component(&grid, x, y, &mut used, &mut cells);

        let (x0, y0) = cells
            .iter()
            .fold((10000, 10000), |(xx, yy), (x, y)| (xx.min(*x), yy.min(*y)));
        let (x1, y1) = cells
            .iter()
            .fold((0, 0), |(xx, yy), (x, y)| (xx.max(*x), yy.max(*y)));
        let (x1, y1) = (x1 + 1, y1 + 1);

        if (x0, y0) == (0, 0) {
            continue;
        }

        let mut comp = vec![vec![false; y1 - y0]; x1 - x0];
        for (i, j) in cells {
            comp[i - x0][j - y0] = true;
        }
        let mut add = vec![];
        // num / var
        if (y0 + 1..y1).all(|j| grid[x0][j]) && (x0 + 1..x1).all(|i| grid[i][y0]) {
            for (i, j) in iproduct!(x0..x1, y0..y1) {
                comp[i - x0][j - y0] = grid[i][j];
                if grid[i][j] {
                    add.push((i, j));
                }
            }
        }

        if let Some(k) = parse_glyph(&comp, false) {
            res.push(Glyph {
                rows: x0..x1,
                cols: y0..y1,
                k,
            });
            for (i, j) in add {
                used[i][j] = true;
            }
        }
    }
    Ok(res)
}

fn output(w: &mut impl std::io::Write, grid: &Grid) -> Result<(), std::io::Error> {
    grid.iter().try_for_each(|row| {
        writeln!(
            w,
            "{}",
            row.iter()
                .map(|v| if *v { "1" } else { "0" })
                .collect::<Vec<&str>>()
                .join("")
        )
    })
}

fn parse_txt(file: &str) -> Result<Grid, Error> {
    let mut buf = String::new();
    std::fs::File::open(file)?.read_to_string(&mut buf)?;
    buf.split('\n')
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '0' => Ok(false),
                    '1' => Ok(true),
                    _ => Err(Error::from(format!("invalid char {}", c))),
                })
                .collect()
        })
        .collect()
}

fn parse_img(file: &str) -> Result<Grid, Error> {
    let img = image::open(file)?;
    let img = img.into_rgb();

    let H = img.height();
    let W = img.width();

    let (x0, y0) = iproduct!(0..W, 0..H)
        .find(|(x, y)| img.get_pixel(*x, *y)[0] > 250)
        .ok_or(Error::from("white not found"))?;
    let SZ = (0..)
        .find(|i| img.get_pixel(x0 + i, y0 + i)[0] < 5)
        .ok_or(Error::from("black not found"))?;
    let (x1, y1) = iproduct!((0..W).rev(), (0..H).rev())
        .find(|(x, y)| img.get_pixel(*x, *y)[0] > 250)
        .ok_or(Error::from("white not found"))?;
    let (x1, y1) = (x1 + 1, y1 + 1);

    Ok((y0..y1)
        .step_by(SZ as usize)
        .map(|y| {
            (x0..x1)
                .step_by(SZ as usize)
                .map(|x| {
                    if img.get_pixel(x, y)[0] < 128 {
                        false
                    } else {
                        true
                    }
                })
                .collect()
        })
        .collect())
}

fn parse_file(file: Option<&str>) -> Result<Grid, Error> {
    let mut temp;
    let file = if let Some(x) = file {
        x
    } else {
        temp = tempfile::Builder::new().suffix(".png").tempfile()?;
        std::io::copy(&mut std::io::stdin().lock(), &mut temp)?;
        temp.path()
            .to_str()
            .ok_or(Error::from("no tempfile path"))?
    };
    if let Ok(grid) = parse_img(file) {
        return Ok(grid);
    }
    parse_txt(file)
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_test() {
        assert_eq!(2, 1 + 1);
    }
}
