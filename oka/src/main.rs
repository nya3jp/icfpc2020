#![allow(non_snake_case, unused)]

extern crate clap;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate tempfile;
#[macro_use]
extern crate itertools;

use clap::{App, Arg, SubCommand};
use std::io::Read;

type Error = Box<dyn std::error::Error>;
type Writer = Box<dyn std::io::Write>;

fn main() {
    run().unwrap_or_else(|s| eprintln!("{}", s))
}

fn writer(file: Option<&str>) -> Result<Writer, Error> {
    Ok(if let Some(f) = file {
        Box::new(std::fs::File::create(f)?)
    } else {
        Box::new(std::io::stdout())
    })
}

fn run() -> Result<(), Error> {
    let matches = App::new("oka")
        .version("1.0")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("output file name")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("png2txt")
                .about("parse given png image to txt format and output to stdout")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .index(1),
                ),
            SubCommand::with_name("annotate")
                .about("annotate the given program file (png or txt) and output to stdout")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .index(1),
                ),
        ])
        .get_matches();

    let mut out = writer(matches.value_of("output"))?;

    match matches.subcommand() {
        ("png2txt", Some(m)) => {
            let grid = parse_file(m.value_of("INPUT"))?;
            output(&mut out, &grid)?;
        }
        ("annotate", Some(m)) => {
            let grid = parse_file(m.value_of("INPUT"))?;
            let glyphs = parse(&grid)?;
            output_svg(&mut out, &grid, &glyphs)?;
        }
        _ => return Err("No such subcommand".into()),
    };
    Ok(())
}

fn output_svg(w: &mut Writer, grid: &Grid, glyphs: &Vec<Glyph>) -> Result<(), std::io::Error> {
    const SZ: usize = 10;
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
                                r#"<rect y="{}" x="{}" height="{}" width="{}" fill="white"/>
    "#,
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
                Kind::Int(_) | Kind::Var(_) => "yellow",
                Kind::Ellipsis => "black",
                Kind::Unknown(_) => "red",
                _ => "green",
            };
            format!(
                r#"<rect y="{}" x="{}" height="{}" width="{}" fill="{}" fill-opacity="0.4"/>
<text y="{}" x="{}" dominant-baseline='middle' text-anchor='middle' fill='white' style='paint-order: stroke; fill: white; stroke: black; stroke-width: 2px; font-size: 5px'>{}</text>
"#,
            x1,
            y1,
            (x2-x1),
            (y2-y1),
            color,
            (x1+x2)/2,
            (y1+y2)/2,
            g.k.to_string(),
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
    Ellipsis,
    Int(isize),
    Var(usize),
    Equal,
    Apply,
    Inc,
    Dec,
    Plus,
    Molecule(&'static str),
    Unknown(isize),
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Kind::Ellipsis => "...".into(),
            Kind::Int(n) => format!(
                "{}{}",
                n,
                if 1 <= *n && (*n as usize) - 1 < ELEMENTS.len() {
                    format!("({})", ELEMENTS[*n as usize - 1])
                } else {
                    "".into()
                }
            ),
            Kind::Var(i) => format!("x{}", i),
            Kind::Equal => "==".into(),
            Kind::Apply => "ap".into(),
            Kind::Inc => "inc".into(),
            Kind::Dec => "dec".into(),
            Kind::Plus => "+".into(),
            Kind::Molecule(s) => s.to_string(),
            Kind::Unknown(x) => format!("?{}", x),
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
        m.insert(12, Kind::Equal);
        m.insert(365, Kind::Plus);
        m.insert(401, Kind::Dec);
        m.insert(417, Kind::Inc);
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

        m.insert(56, Kind::Molecule("-NH-C(R)H-C(=O)-"));
        m.insert(20, Kind::Molecule("-CH(NH2)-CH3"));
        m.insert(29, Kind::Molecule("-CH2-CH3"));
        m.insert(30, Kind::Molecule("-CH2-NH2"));
        m.insert(31, Kind::Molecule("Ser"));
        m.insert(44, Kind::Molecule("Thr"));
        m.insert(45, Kind::Molecule("-CH2-CH2-OH"));
        m.insert(58, Kind::Molecule("Asn"));

        m.insert(59, Kind::Molecule("-CH2-C(=O)-OH"));
        m.insert(72, Kind::Molecule("-C2H4-C(=O)-NH2"));
        m.insert(9, Kind::Molecule("-C2H4-C(=O)-OH"));
        m.insert(84, Kind::Molecule("-CH2-CH(-CH3)2"));
        m.insert(86, Kind::Molecule("-CH2-CH2-NH-C(NH2)(NH)"));
        m.insert(87, Kind::Molecule("-CH2-CH2-CH2-COOH"));
        m.insert(100, Kind::Molecule("-CH2-CH2-CH2-NH-C(NH2)(NH)"));
        m.insert(118, Kind::Molecule("-CH2-C6H5"));
        m.insert(134, Kind::Molecule("-CH2-C6H4-OH"));
        m
    };
}

fn glyph(grid: &Grid, x: usize, y: usize, flip: bool) -> Result<Glyph, Error> {
    let inside = |i: usize, j: usize| x + i < grid.len() && y + j < grid[0].len();
    let get = |i: usize, j: usize| inside(i, j) && (grid[x + i][y + j] ^ flip);

    let gen_glyph = |h: usize, w: usize, k: Kind| {
        Ok(Glyph {
            rows: x..x + h,
            cols: y..y + w,
            k,
        })
    };

    let mut n = 0;
    while get(0, n + 1) && get(n + 1, 0) {
        n += 1;
    }
    if !inside(0, n + 1) || !inside(n + 1, 0) {
        return gen_glyph(1, 1, Kind::Unknown(-1));
    }
    if n == 0 {
        return gen_glyph(1, 1, Kind::Unknown(-1));
    }
    if n > 8 {
        return gen_glyph(1, 1, Kind::Unknown(-1));
    }

    let is_var = (0..=n).all(|i| get(i, 0) && get(i, n) && get(0, i) && get(n, i));
    if is_var {
        return match glyph(grid, x + 1, y + 1, !flip)?.k {
            Kind::Int(i) => {
                if i < 0 {
                    gen_glyph(1, 1, Kind::Unknown(-1))
                } else {
                    Ok(Glyph {
                        rows: x..x + n + 1,
                        cols: y..y + n + 1,
                        k: Kind::Var(i as usize),
                    })
                }
            }
            _ => gen_glyph(1, 1, Kind::Unknown(-1)),
            // _ => Err("invalid glyph".into()),
        };
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
        return Ok(Glyph {
            rows: x..x + n + 1 + (if negative { 1 } else { 0 }),
            cols: y..y + n + 1,
            k: Kind::Int(num),
        });
    }

    Ok(Glyph {
        rows: x..x + n + 1,
        cols: y..y + n + 1,
        k: if let Some(k) = NUM_TO_KIND.get(&num) {
            k.clone()
        } else {
            Kind::Unknown(num)
        },
    })
}

fn parse_glyph(x: usize, y: usize, grid: &Grid) -> Result<Glyph, Error> {
    dbg!(x, y);
    // ellipsis
    if (y..y + 7).all(|yy| grid[x].get(yy) == Some(&((yy - y) % 2 == 0))) {
        return Ok(Glyph {
            rows: x..x + 1,
            cols: y..y + 7,
            k: Kind::Ellipsis,
        });
    }
    glyph(&grid, x, y, false)
}

fn parse(grid: &Grid) -> Result<Vec<Glyph>, Error> {
    let (h, w) = (grid.len(), grid[0].len());
    let mut used = vec![vec![false; w]; h];
    let mut res = vec![];
    for x in 1..h - 2 {
        for y in 1..w - 2 {
            if used[x][y] {
                continue;
            }
            if grid[x][y] || grid[x][y + 1] && grid[x + 1][y] {
                let g = parse_glyph(x, y, &grid)?;
                for i in g.rows.clone() {
                    for j in g.cols.clone() {
                        used[i][j] = true;
                    }
                }
                res.push(g);
            }
        }
    }
    Ok(res)
}

fn output(w: &mut Writer, grid: &Grid) -> Result<(), std::io::Error> {
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
                    '1' => Ok(false),
                    _ => Err(Error::from(format!("invalid char {}", c))),
                })
                .collect()
        })
        .collect()
}

fn parse_img(file: &str) -> Result<Grid, Error> {
    let img = image::open(file)?;
    let img = img.into_rgb();

    // const SZ: usize = 4;
    let H = img.height();
    let W = img.width();

    let (x0, y0) = iproduct!(0..W, 0..H)
        .find(|(x, y)| img.get_pixel(*x, *y)[0] > 250)
        .ok_or(Error::from("white not found"))?;
    let SZ = (0..)
        .find(|i| img.get_pixel(x0 + i, y0 + i)[0] < 5)
        .ok_or(Error::from("black not found"))?;
    let (mut x1, mut y1) = iproduct!((0..W).rev(), (0..H).rev())
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
