#![allow(non_snake_case, unused, non_upper_case_globals)]

extern crate clap;
#[macro_use]
extern crate itertools;

use std::fs::File;
use std::io::Read;
use std::{
    io::{BufReader, BufWriter},
    str::FromStr,
};

type Writer = Box<dyn std::io::Write>;
type Reader = Box<dyn std::io::Read>;
type Error = Box<dyn std::error::Error>;

fn main() {
    let child = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || run().unwrap())
        .unwrap();
    child.join().unwrap();
}

fn writer(file: Option<&str>) -> Result<Writer, Error> {
    Ok(if let Some(f) = file {
        Box::new(std::fs::File::create(f)?)
    } else {
        Box::new(std::io::stdout())
    })
}

fn reader(file: Option<&str>) -> Result<Reader, Error> {
    Ok(if let Some(f) = file {
        Box::new(std::fs::File::open(f)?)
    } else {
        Box::new(std::io::stdin())
    })
}

fn run() -> Result<(), Error> {
    let m = clap::App::new("modulate")
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input file name")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .help("output file name")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("demodulate")
                .short("d")
                .long("demodulate")
                .help("demodulate given list"),
        )
        .get_matches();

    let mut w = writer(m.value_of("output"))?;
    let mut r = reader(m.value_of("input"))?;

    let demod = m.is_present("demodulate");

    let mut s = String::new();
    r.read_to_string(&mut s);

    if demod {
        let val = demodulate(&mut s.trim().chars().map(|c| c == '1'))
            .ok_or(Error::from("demodulate failed"))?;
        println!("{}", val.to_string());
    } else {
    }

    Ok(())
}

#[derive(Debug)]
enum Value {
    Int(i64),
    Nil,
    Cons(Box<Value>, Box<Value>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            &Value::Int(n) => format!("{}", n),
            Value::Nil => format!("nil"),
            Value::Cons(x, y) => format!("({}, {})", x.to_string(), y.to_string()),
        }
    }
}

// impl FromStr for Value {
//     type Err = Error;
//     // fn from_str(mut s: &str) -> Result<Self, Self::Err> {
//     // let s = s.trim();
//     // let s = s.as_bytes();
//     // match s[0] {
//     // b'(' => from_str()
//     // _
//     // }
//     // todo!()
//     // }
// }

fn vec_to_str(v: &Vec<bool>) -> String {
    v.iter().map(|x| if *x { "1" } else { "0" }).collect()
}

fn vec_from_str(s: &str) -> Vec<bool> {
    s.chars().map(|x| x == '1').collect()
}

fn modulate_to_string(val: &Value) -> String {
    let mut v = vec![];
    modulate(&val, &mut v);
    vec_to_str(&v)
}

fn modulate(val: &Value, v: &mut Vec<bool>) {
    match val {
        &Value::Int(n) => {
            let n = if n >= 0 {
                v.push(false);
                v.push(true);
                n as u64
            } else {
                v.push(false);
                v.push(true);
                n.abs() as u64
            };

            let keta = 64 - n.leading_zeros();
            let t = (keta + 3) / 4;

            for _ in 0..t {
                v.push(true);
            }
            v.push(false);

            for i in (0..4 * t).rev() {
                v.push((n >> i) & 1 == 1);
            }
        }
        Value::Nil => {
            v.push(false);
            v.push(false);
        }
        Value::Cons(hd, tl) => {
            v.push(true);
            v.push(true);
            modulate(hd, v);
            modulate(tl, v);
        }
    }
}

fn demodulate(it: &mut impl Iterator<Item = bool>) -> Option<Value> {
    let t0 = it.next()?;
    let t1 = it.next()?;

    Some(match (t0, t1) {
        (false, false) => Value::Nil,
        (true, true) => {
            let x = demodulate(it)?;
            let y = demodulate(it)?;
            Value::Cons(Box::new(x), Box::new(y))
        }
        _ => {
            let mut t = 0;
            while it.next()? {
                t += 1;
            }
            let mut v = 0;
            for i in (0..4 * t).rev() {
                v |= (if it.next()? { 1 } else { 0 }) << i;
            }
            Value::Int(v)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_demod() {
        let bin = "110110000111011111100001001111110100110000";
        let lst = "(1, (81740, nil))";

        assert_eq!(
            lst,
            demodulate(&mut vec_from_str(bin).into_iter())
                .unwrap()
                .to_string()
                .as_str()
        );
    }

    fn test_mod() {
        let bin = "110110000111011111100001001111110100110000";
        let lst = "(1, (81740, nil))";

        // assert_eq!(modulate_to_string(), 2);
    }
}
