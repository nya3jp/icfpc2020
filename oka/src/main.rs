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
        .arg(
            clap::Arg::with_name("decimal")
                .short("d")
                .long("decimal")
                .help("use decimal notation in demodulation"),
        )
        .get_matches();

    let mut w = writer(m.value_of("output"))?;
    let mut r = reader(m.value_of("input"))?;

    let demod = m.is_present("demodulate");

    let mut s = String::new();
    r.read_to_string(&mut s);

    if demod {
        if m.is_present("decimal") {
            let i = i64::from_str(&s)?;
            s = format!("{:b}", i);
            dbg!(&s);
        }
        let val = demodulate(&mut s.trim().chars().map(|c| c == '1'))
            .ok_or(Error::from("demodulate failed"))?;
        println!("{}", val.to_string());
    } else {
        let val = Value::from_str(&s)?;
        let v = modulate_to_string(&val);
        println!("{}", v);
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

impl FromStr for Value {
    type Err = Error;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        Value::from_iter(&mut s.bytes().peekable())
    }
}

impl Value {
    fn from_iter<I: Iterator<Item = u8>>(i: &mut std::iter::Peekable<I>) -> Result<Self, Error> {
        let b: u8 = i.next().ok_or(Error::from("iterator exhausted"))?;
        match b {
            b'(' => {
                let x = Value::from_iter(i)?;
                i.find(|c| *c == b',').ok_or(Error::from(", not found"))?;
                let y = Value::from_iter(i)?;
                i.find(|c| *c == b')').ok_or(Error::from(") not found"))?;
                Ok(Value::Cons(Box::new(x), Box::new(y)))
            }
            b'-' | b'0'..=b'9' => {
                let neg = b == b'-';
                let mut x: i64 = if neg { 0 } else { (b - b'0') as i64 };
                loop {
                    if let Some(c) = i.peek() {
                        if b'0' <= *c && *c <= b'9' {
                            x = x * 10 + (*c - b'0') as i64;
                            i.next().unwrap();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if neg {
                    x = -x;
                }
                Ok(Value::Int(x))
            }
            b'n' => {
                i.next();
                i.next();
                Ok(Value::Nil)
            }
            b' ' => Value::from_iter(i),
            _ => Err(Error::from(format!("parse fail {}", b as char))),
        }
    }
}

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
                v.push(true);
                v.push(false);
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
        (_, y) => {
            let mut t = 0;
            while it.next()? {
                t += 1;
            }
            let mut v = 0;
            for i in (0..4 * t).rev() {
                v |= (if it.next()? { 1 } else { 0 }) << i;
            }
            Value::Int(if y { v } else { -v })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_demod() {
        for tc in [
            (
                "110110000111011111100001001111110100110000",
                "(1, (81740, nil))",
            ),
            ("010", "0"),
            ("01100001", "1"),
            ("10100001", "-1"),
        ]
        .iter()
        {
            let bin = tc.0;
            let lst = tc.1;

            assert_eq!(
                lst,
                demodulate(&mut vec_from_str(bin).into_iter())
                    .unwrap()
                    .to_string()
                    .as_str()
            );
        }
    }

    #[test]
    fn test_mod() {
        for tc in [
            (
                "110110000111011111100001001111110100110000",
                "(1, (81740, nil))",
            ),
            ("010", "0"),
            ("01100001", "1"),
            ("10100001", "-1"),
        ]
        .iter()
        {
            let bin = tc.0;
            let lst = tc.1;

            let val = Value::from_str(lst).unwrap();
            let v = modulate_to_string(&val);
            assert_eq!(&v, bin);
        }
    }

    #[test]
    fn test_mod_demod() {
        let lst = "(1, (-81740, nil))";

        let val = Value::from_str(lst).unwrap();
        let v = modulate_to_string(&val);
        assert_eq!(
            lst,
            demodulate(&mut vec_from_str(&v).into_iter())
                .unwrap()
                .to_string()
                .as_str()
        );
    }
}
