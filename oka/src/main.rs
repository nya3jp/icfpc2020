#![allow(non_snake_case, unused)]

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
            let grid = parse_file(m.value_of("INPUT"))?;
            let glyphs = parse(&grid)?;

            output_svg(&mut w, &grid, &glyphs)?;
        }
        _ => return Err("No such subcommand".into()),
    };
    Ok(())
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
                Kind::Int(_) | Kind::Var(_) => "#004D40",
                Kind::Molecule(_) => "#01579B",
                Kind::Unnamed(_) => "#BF360C",
                _ => "#827717",
            };
            format!(
                r#"<rect y="{}" x="{}" height="{}" width="{}" fill="{}" fill-opacity="0.5"/>
<text y="{}" x="{}" dominant-baseline='middle' text-anchor='middle' fill='white' style='paint-order: stroke; fill: white; stroke: black; stroke-width: 1px; font-family="monospace"; font-size: 13px'>{}</text>
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
    Var(usize),
    Equal,
    Apply,
    Inc,
    Dec,
    Plus,
    Molecule(&'static str),
    Life(&'static str),
    Unnamed(&'static str),
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
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

        m.insert(56, Kind::Molecule("-NH-\nC(R)H-\nC(=O)-"));
        m.insert(20, Kind::Molecule("-CH(NH2)-\nCH3"));
        m.insert(29, Kind::Molecule("-CH2-\nCH3"));
        m.insert(30, Kind::Molecule("-CH2-\nNH2"));
        m.insert(31, Kind::Molecule("Ser"));
        m.insert(44, Kind::Molecule("Thr"));
        m.insert(45, Kind::Molecule("-CH2-\nCH2-\nOH"));
        m.insert(58, Kind::Molecule("Asn"));

        m.insert(59, Kind::Molecule("-CH2-\nC(=O)-\nOH"));
        m.insert(72, Kind::Molecule("-C2H4-\nC(=O)-\nNH2"));
        m.insert(9, Kind::Molecule("-C2H4-\nC(=O)-\nOH"));
        m.insert(84, Kind::Molecule("-CH2-\nCH(-CH3)2"));
        m.insert(86, Kind::Molecule("-CH2-\nCH2-NH-\nC(NH2)(NH)"));
        m.insert(87, Kind::Molecule("-CH2-\nCH2-CH2-\nCOOH"));
        m.insert(100, Kind::Molecule("-CH2-CH2-\nCH2-NH-\nC(NH2)(NH)"));
        m.insert(118, Kind::Molecule("-CH2-\nC6H5"));
        m.insert(134, Kind::Molecule("-CH2-\nC6H4-OH"));
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
            Kind::Molecule("Ser life\nRNA")
        ),
        (
            vec!["1000", "0111", "0010", "0111"],
            Kind::Molecule("Ser life\nbeta sheet")
        ),
        (vec!["1000", "0111", "0111", "0101"], Kind::Life("Asn life")),
        (
            vec!["1001", "0111", "0101", "0101"],
            Kind::Molecule("Asn life\nRNA")
        ),
        (
            vec!["1000", "0111", "0101", "0111"],
            Kind::Molecule("Asn life\nbeta sheet")
        ),
        (
            vec!["1010", "0101", "1101", "0010"],
            Kind::Molecule("Ser life\nside-chain")
        ),
        (
            vec!["1010", "0101", "1101", "0101"],
            Kind::Molecule("Asn life\nside-chain")
        ),
        (
            vec!["1011", "0111", "1111", "0010"],
            Kind::Molecule("Ser life\nprotein")
        ),
        (
            vec!["1011", "0111", "1111", "0101"],
            Kind::Molecule("Asn life\nprotein")
        ),
        (
            vec!["1000", "0111", "0111", "0111"],
            Kind::Life("Mother\nplanet")
        ),
        (vec!["1011", "0111", "1111", "1111"], Kind::Unnamed("13-1")),
        (vec!["1010", "0111", "0101", "0111"], Kind::Unnamed("14-1")),
    ];
}

fn parse_glyph(comp: &Grid, x: usize, y: usize, flip: bool) -> Option<Kind> {
    let inside = |i: usize, j: usize| x + i < comp.len() && y + j < comp[0].len();
    let get = |i: usize, j: usize| inside(i, j) && (comp[x + i][y + j] ^ flip);

    for (fig, k) in FIXED.iter() {
        let mut ok = true;
        if fig.iter().enumerate().all(|(i, row)| {
            row.chars()
                .enumerate()
                .all(|(j, c)| (c == '1') == get(i, j))
        }) {
            return Some(k.clone());
        }
    }

    if !((1..comp.len() - x).all(|i| get(i, 0)) && (1..comp[0].len() - y).all(|j| get(0, j))) {
        return None;
    }
    let n = comp[0].len() - y - 1;
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
        return match parse_glyph(&comp, x + 1, y + 1, !flip)? {
            Kind::Int(i) => {
                if i < 0 {
                    None
                } else {
                    Some(Kind::Var(i as usize))
                }
            }
            _ => None,
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
        if (laboratory() == Lab::Pflockingen) && (num == 0) {
            return None;
        }
        return Some(Kind::Int(num));
    }

    NUM_TO_KIND.get(&num).map(Clone::clone)
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

        if let Some(k) = parse_glyph(&comp, 0, 0, false) {
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
