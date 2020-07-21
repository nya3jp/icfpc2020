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

use std::collections::{BTreeMap, VecDeque};
use std::{
    cell::RefCell,
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
    rc::Rc,
};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Dictionary = BTreeMap<String, ExprRef>;
type ExprRef = Rc<RefCell<Expr>>;
type ExprValueRef = Rc<RefCell<ExprValue>>;

#[derive(Debug)]
pub enum ExprValue {
    Lam(String, ExprRef),
    App(ExprRef, ExprRef),
    Atom(String),
    Int(i64),
}

impl ExprValue {
    fn atom(&self) -> Option<&str> {
        match self {
            ExprValue::Atom(s) => Some(&s),
            _ => None,
        }
    }

    fn int(&self) -> Option<i64> {
        match self {
            ExprValue::Int(s) => Some(*s),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Expr {
    value: ExprValueRef,
    evaluated: Option<ExprRef>,
}

impl Expr {
    pub fn new(v: ExprValue) -> ExprRef {
        Rc::new(RefCell::new(Expr {
            value: Rc::new(RefCell::new(v)),
            evaluated: None,
        }))
    }

    pub fn make_atom(s: &str) -> ExprRef {
        Expr::new(ExprValue::Atom(s.to_string()))
    }

    pub fn make_int(n: i64) -> ExprRef {
        let ret = Expr::new(ExprValue::Int(n));
        ret.borrow_mut().evaluated = Some(Rc::clone(&ret));
        ret
    }

    pub fn make_app(f: ExprRef, x: ExprRef) -> ExprRef {
        Expr::new(ExprValue::App(f, x))
    }

    // fn print(&self) -> String {
    //     match self.value {
    //         Expr::App(f, x) => format!("({} {})", f.print(), x.print()),
    //         Expr::Atom(f) => format!("<atom:{}>", f),
    //         Expr::Int(lit) => lit.print(),
    //     }
    // }

    // fn to_scheme(&self) -> String {
    //     match self {
    //         Expr::Lam(var, x) => format!("\\{} -> {}", var, x.to_scheme()),
    //         Expr::App(f, x) => format!("({} {})", f.to_scheme(), x.to_scheme()),
    //         Expr::Lit(lit) => match lit {
    //             Literal::Int(n) => format!("{}", n),
    //             Literal::Var(v) => format!("({})", v),
    //             _ => unreachable!(),
    //         },
    //         _ => unreachable!(),
    //     }
    // }

    // fn cexpr(&self) -> Expr {
    //     match self {
    //         Expr::Lit(lit) => Expr::Lit(lit.clone()),
    //         Expr::App(f, x) => Expr::make_app(f.cexpr(), x.cexpr()),
    //         Expr::Lam(var, x) => {
    //             if !x.contains(var) {
    //                 Expr::make_app(Expr::make_var("k"), x.cexpr())
    //             } else if x.var() == Some(&var) {
    //                 Expr::make_var("i")
    //             } else {
    //                 match x.as_ref() {
    //                     Expr::Lam(_, _) => Expr::Lam(var.to_string(), Box::new(x.cexpr())).cexpr(),
    //                     Expr::App(f, x) => {
    //                         let fc = f.contains(var);
    //                         let xc = x.contains(var);

    //                         if fc && xc {
    //                             Expr::make_app(
    //                                 Expr::make_app(
    //                                     Expr::make_var("s"),
    //                                     Expr::Lam(var.to_string(), f.clone()).cexpr(),
    //                                 ),
    //                                 Expr::Lam(var.to_string(), x.clone()).cexpr(),
    //                             )
    //                         } else if fc {
    //                             Expr::make_app(
    //                                 Expr::make_app(
    //                                     Expr::make_var("c"),
    //                                     Expr::Lam(var.to_string(), f.clone()).cexpr(),
    //                                 ),
    //                                 x.cexpr(),
    //                             )
    //                         } else if xc {
    //                             Expr::make_app(
    //                                 Expr::make_app(Expr::make_var("b"), f.cexpr()),
    //                                 Expr::Lam(var.to_string(), x.clone()).cexpr(),
    //                             )
    //                         } else {
    //                             unreachable!()
    //                         }
    //                     }
    //                     _ => unreachable!(),
    //                 }
    //             }
    //         }
    //         _ => unreachable!(),
    //     }
    // }

    // fn contains(&self, var: &str) -> bool {
    //     match self {
    //         Expr::Lit(lit) => match lit {
    //             Literal::Var(v) => v == var,
    //             _ => false,
    //         },
    //         Expr::App(f, x) => f.contains(var) || x.contains(var),
    //         Expr::Lam(v, x) => {
    //             if v == var {
    //                 false
    //             } else {
    //                 x.contains(var)
    //             }
    //         }
    //         _ => unreachable!(),
    //     }
    // }

    // fn eval(&self, dict: &BTreeMap<String, Expr>) -> Expr {
    //     // eprintln!("{:?}", self.print());
    //     match self {
    //         Expr::Lam(_, _) => unreachable!(),
    //         Expr::App(f, x) => {
    //             let f = f.eval(dict);
    //             match f {
    //                 Expr::Builtin(f, arity, mut xs) => {
    //                     xs.push(x.as_ref().clone());
    //                     if xs.len() == arity {
    //                         eval_builtin(&f, &xs, dict)
    //                     } else {
    //                         Expr::Builtin(f, arity, xs)
    //                     }
    //                 }
    //                 _ => unreachable!("Invalid app: {:?} {:?}", f.print(), x.print()),
    //             }
    //         }
    //         Expr::Builtin(_, _, _) => self.clone(),
    //         Expr::Lit(lit) => match lit {
    //             Literal::Int(_n) => self.clone(),
    //             Literal::Var(var) => {
    //                 if let Some(e) = dict.get(var.as_str()) {
    //                     e.eval(dict)
    //                 } else {
    //                     panic!("variable: {} is not defined", var)
    //                 }
    //             }
    //             Literal::Canvas(_) => self.clone(),
    //         },
    //         Expr::Thunk(t) => {
    //             let mut b = t.borrow_mut();
    //             let c = b.eval(dict);
    //             *b = c.clone();
    //             c
    //         }
    //     }
    // }

    // fn to_value(self, dict: &BTreeMap<String, Expr>) -> Value {
    //     if let Some(n) = self.int() {
    //         Value::int(n)
    //     } else {
    //         match self {
    //             Expr::Lit(Literal::Canvas(ps)) => Value::Canvas(ps),
    //             _ => {
    //                 let t = Expr::Thunk(Rc::new(RefCell::new(self))).eval(dict);
    //                 let b = Expr::make_apps(
    //                     Expr::make_var("isnil"),
    //                     vec![t.clone(), Expr::make_int(1), Expr::make_int(0)],
    //                 )
    //                 .eval(dict);

    //                 let b = b.int().unwrap();

    //                 if b == 1 {
    //                     Value::Nil
    //                 } else {
    //                     let car = Expr::make_app(Expr::make_var("car"), t.clone()).eval(dict);
    //                     let cdr = Expr::make_app(Expr::make_var("cdr"), t).eval(dict);
    //                     Value::cons(car.to_value(dict), cdr.to_value(dict))
    //                 }
    //             }
    //         }
    //     }
    // }
}

mod dsl {
    use super::{Expr, ExprRef};

    pub fn app(f: ExprRef, x: ExprRef) -> ExprRef {
        Expr::make_app(f, x)
    }

    pub fn atom(name: &str) -> ExprRef {
        Expr::make_atom(name)
    }

    pub fn tt() -> ExprRef {
        Expr::make_atom("t")
    }

    pub fn ff() -> ExprRef {
        Expr::make_atom("f")
    }

    pub fn bool(b: bool) -> ExprRef {
        if b {
            tt()
        } else {
            ff()
        }
    }

    pub fn int(n: i64) -> ExprRef {
        Expr::make_int(n)
    }
}

fn as_num(x: ExprRef, dict: &Dictionary) -> Option<i64> {
    eval(x, dict).borrow().value.borrow().int()
}

fn eval(e: ExprRef, dict: &Dictionary) -> ExprRef {
    if let Some(e) = &e.borrow().evaluated {
        return Rc::clone(&e);
    }

    let init_expr = Rc::clone(&e);
    let mut e = e;

    loop {
        let res = try_eval(Rc::clone(&e), dict);
        if Rc::ptr_eq(&res, &e) {
            init_expr.borrow_mut().evaluated = Some(Rc::clone(&res));
            return res;
        }
        e = res;
    }
}

fn try_eval(e: ExprRef, dict: &Dictionary) -> ExprRef {
    use dsl::*;

    let eval_cons = |a, b| {
        let ret = app(app(atom("cons"), eval(a, dict)), eval(b, dict));
        ret.borrow_mut().evaluated = Some(Rc::clone(&ret));
        ret
    };

    let num = |fname, x| {
        as_num(x, dict).unwrap_or_else(|| panic!(format!("{}: argument is not int", fname)))
    };

    if let Some(e) = &e.borrow().evaluated {
        return Rc::clone(e);
    }

    if let Some(name) = e.borrow().value.borrow().atom() {
        if let Some(ret) = dict.get(name) {
            return Rc::clone(ret);
        }
    }

    if let Some(_) = Rc::clone(&e).borrow().value.borrow().int() {
        return e;
    }

    let bf = e.borrow();
    let bbf = bf.value.borrow();
    if let ExprValue::App(f, x) = &*bbf {
        let f = eval(Rc::clone(f), dict);
        let x = Rc::clone(x);

        match f.borrow().value.borrow().atom() {
            Some("neg") => return Expr::make_int(-num("neg", x)),
            Some("i") => return x,
            Some("nil") => return tt(),
            Some("isnil") => return app(x, app(tt(), app(tt(), ff()))),
            Some("car") => return app(x, tt()),
            Some("cdr") => return app(x, ff()),
            _ => {}
        }

        let bf = f.borrow();
        let bbf = bf.value.borrow();
        if let ExprValue::App(f, y) = &*bbf {
            let f = eval(Rc::clone(f), dict);
            let y = Rc::clone(y);

            match f.borrow().value.borrow().atom() {
                Some("t") => return y,
                Some("f") => return x,
                Some("add") => return int(num("add", y) + num("add", x)),
                Some("mul") => return int(num("mul", y) * num("mul", x)),
                Some("div") => return int(num("div", y) / num("div", x)),
                Some("lt") => return bool(num("lt", y) < num("lt", x)),
                Some("eq") => return bool(num("eq", y) == num("eq", x)),
                Some("cons") => return eval_cons(y, x),
                _ => {}
            }

            let bf = f.borrow();
            let bbf = bf.value.borrow();
            if let ExprValue::App(f, z) = &*bbf {
                let f = eval(Rc::clone(f), dict);
                let z = Rc::clone(z);

                let bf = f.borrow();
                let bbf = bf.value.borrow();

                match bbf.atom() {
                    Some("s") => return app(app(z, Rc::clone(&x)), app(y, x)),
                    Some("c") => return app(app(z, x), y),
                    Some("b") => return app(z, app(y, x)),
                    Some("cons") => return app(app(x, z), y),

                    Some(f) => panic!("invalid function: {}", f),
                    _ => panic!(
                        "invalid ap: {:?}, {:?}, {:?}, {:?}",
                        bbf,
                        x.borrow().value,
                        y.borrow().value,
                        z.borrow().value,
                    ),
                }
            }
        }
    }

    drop(bbf);
    drop(bf);
    e
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Int(i64),
    Nil,
    Cons(Box<Value>, Box<Value>),
}

impl Value {
    fn nil() -> Value {
        Value::Nil
    }

    fn cons(a: Value, b: Value) -> Value {
        Value::Cons(Box::new(a), Box::new(b))
    }

    fn int(n: i64) -> Value {
        Value::Int(n)
    }

    fn list(v: Vec<Value>) -> Value {
        let mut ret = Self::nil();
        for v in v.into_iter().rev() {
            ret = Self::cons(v, ret);
        }
        ret
    }

    fn modulate(&self, v: &mut Vec<bool>) {
        match self {
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
                hd.modulate(v);
                tl.modulate(v);
            }
            _ => unreachable!(),
        }
    }

    fn demodulate(it: &mut impl Iterator<Item = bool>) -> Option<Value> {
        let t0 = it.next()?;
        let t1 = it.next()?;

        Some(match (t0, t1) {
            (false, false) => Value::Nil,
            (true, true) => {
                let x = Self::demodulate(it)?;
                let y = Self::demodulate(it)?;
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

    fn print(&self) -> String {
        match self {
            Value::Int(n) => format!("{}", *n),
            Value::Nil => "nil".to_string(),
            Value::Cons(hd, tl) => format!("({} . {})", hd.print(), tl.print()),
        }
    }

    // fn to_expr(&self) -> Expr {
    //     match self {
    //         Value::Int(n) => Expr::make_int(*n),
    //         Value::Nil => Expr::make_var("nil"),
    //         Value::Cons(hd, tl) => {
    //             Expr::make_apps(Expr::make_var("cons"), vec![hd.to_expr(), tl.to_expr()])
    //         }
    //         Value::Canvas(ps) => Expr::Lit(Literal::Canvas(ps.clone())),
    //     }
    // }

    fn to_sexp(&self) -> String {
        match self {
            &Value::Int(n) => format!("{}", n),
            Value::Nil => "()".to_string(),
            Value::Cons(hd, tl) => format!("({} . {})", hd.to_sexp(), tl.to_sexp()),
        }
    }

    fn to_raw(&self) -> String {
        match self {
            Value::Int(n) => format!("{}", *n),
            Value::Nil => "nil".to_string(),
            Value::Cons(hd, tl) => format!("ap ap cons {} {}", hd.to_raw(), tl.to_raw()),
        }
    }
}

fn modulate(v: &Value) -> Vec<bool> {
    let mut ret = vec![];
    v.modulate(&mut ret);
    ret
}

fn demodulate(v: &[bool]) -> Option<Value> {
    let mut it = v.iter().cloned();
    let ret = Value::demodulate(&mut it)?;
    assert!(it.next().is_none());
    Some(ret)
}

fn encode(v: &[bool]) -> String {
    v.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

fn decode(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '1').collect()
}

#[test]
fn test_mod() {
    assert_eq!(modulate(&Value::Nil), vec![false, false]);
}

#[test]
fn test_demod() {
    let t = "1101100001111101100010110110001100110110010000";
    let v = t.chars().map(|c| c == '1').collect::<Vec<_>>();
    let v = demodulate(&v).unwrap();
    // eprintln!("{}", demodulate(&v).unwrap().print());
    // assert_eq!();

    let u = modulate(&v)
        .into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect::<String>();
    assert_eq!(t, u);
}

fn parse<'a>(it: &mut impl Iterator<Item = &'a str>) -> ExprRef {
    use dsl::*;

    let s = it.next().unwrap();
    if s == "ap" {
        let f = parse(it);
        let x = parse(it);
        Expr::make_app(f, x)
    } else if s == "(" {
        let mut v = vec![];
        loop {
            v.push(parse(it));
            let s = it.next().unwrap();
            if s == "," {
                continue;
            } else if s == ")" {
                let mut ret = atom("nil");

                for x in v.into_iter().rev() {
                    ret = app(app(atom("cons"), x), ret);
                }

                break ret;
            } else {
                unreachable!();
            }
        }
    } else {
        if let Ok(n) = s.parse() {
            int(n)
        } else {
            atom(s)
        }
    }
}

fn parse_expr(s: &str) -> ExprRef {
    let mut it = s.split_whitespace();
    let ret = parse(&mut it);
    assert!(it.next().is_none());
    ret
}

fn parse_sexp<'a>(it: &mut Peekable<impl Iterator<Item = &'a str>>) -> Option<Value> {
    let s = it.next()?;

    if s == "(" {
        let mut v = vec![];
        loop {
            let s = it.peek()?;
            if s == &")" {
                assert_eq!(it.next().unwrap(), ")");
                return Some(Value::list(v));
            } else if s == &"." {
                assert_eq!(it.next().unwrap(), ".");
                let mut ret = parse_sexp(it)?;
                let s = it.next()?;
                assert_eq!(s, ")");
                for v in v.into_iter().rev() {
                    ret = Value::cons(v, ret);
                }
                return Some(ret);
            } else {
                v.push(parse_sexp(it)?);
            }
        }
    } else if let Ok(n) = s.parse() {
        Some(Value::int(n))
    } else {
        unreachable!("{}", s);
    }
}

fn parse_sexp_str(s: &str) -> Option<Value> {
    let mut ns = String::new();
    for c in s.chars() {
        if c == '(' || c == ')' || c == '.' {
            ns.push(' ');
            ns.push(c);
            ns.push(' ');
        } else {
            ns.push(c);
        }
    }
    // dbg!(&ns);
    let it = ns.split_whitespace();
    let mut it = it.peekable();
    let ret = parse_sexp(&mut it)?;
    assert!(it.next().is_none());
    Some(ret)
}

#[test]
fn test_parse_sexp() {
    assert_eq!(parse_sexp_str("-123"), Some(Value::int(-123)));
    assert_eq!(parse_sexp_str("()"), Some(Value::nil()));

    assert_eq!(
        parse_sexp_str("(1 . 2)"),
        Some(Value::cons(Value::int(1), Value::int(2)))
    );

    assert_eq!(
        parse_sexp_str("(1 2)"),
        Some(Value::list(vec![Value::int(1), Value::int(2)]))
    );

    assert_eq!(
        parse_sexp_str("(1 2 . 3)"),
        Some(Value::cons(
            Value::int(1),
            Value::cons(Value::int(2), Value::int(3))
        ))
    );

    assert_eq!(
        parse_sexp_str("(1 2 3)"),
        Some(Value::list(vec![
            Value::int(1),
            Value::int(2),
            Value::int(3)
        ]))
    );
}

// curl -X POST "https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=REDACTED" -H "accept: */*" -H "Content-Type: text/plain" -d "111111011000010100000"

fn send_request(req: &str) -> String {
    use std::process::Command;
    let output = Command::new("curl").args(&[
        "-X",
        "POST",
        "https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=REDACTED",
        "-H",
        "accept: */*",
        "-H",
        "Content-Type: text/plain",
        "-d",
    ]).arg(req).output().expect("fail to execute curl");

    String::from_utf8(output.stdout).unwrap()
}

fn send_request_value(v: &Value) -> Value {
    demodulate(&decode(&send_request(&encode(&modulate(v))))).expect("demodulate failed")
}

fn plot(ps: &[Vec<(i64, i64)>], html: bool) {
    let mut minx = i64::max_value();
    let mut maxx = i64::min_value();
    let mut miny = i64::max_value();
    let mut maxy = i64::min_value();

    for r in ps.iter() {
        for &(x, y) in r.iter() {
            minx = min(minx, x);
            maxx = max(maxx, x);
            miny = min(miny, y);
            maxy = max(maxy, y);
        }
    }

    if !html {
        let mut bd = vec![vec!['.'; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];

        for (ch, r) in ps.iter().enumerate().rev() {
            for &(x, y) in r.iter() {
                bd[(y - miny) as usize][(x - minx) as usize] = (b'1' + ch as u8) as char;
            }
        }

        let pict = bd
            .into_iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>();

        println!("{} {}", minx, miny);

        print!("       ");
        for x in minx..=maxx {
            print!("{:4}", x);
        }
        println!();

        for (i, row) in pict.iter().enumerate() {
            print!("{:4} | ", miny + i as i64);
            for c in row.chars() {
                print!("   {}", c);
            }
            println!();
        }
        println!("=====");
    } else {
        eprintln!(
            r#"
<table border="0" cellpadding="0" cellspacing="0" width="1024" height="768" bgcolor=" #fdfdfd">
"#
        );

        let mut bd = vec![vec![0; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];

        for (ch, r) in ps.iter().enumerate().rev() {
            for &(x, y) in r.iter() {
                bd[(y - miny) as usize][(x - minx) as usize] += 1 << ch;
            }
        }

        for (i, row) in bd.iter().enumerate() {
            eprintln!("<tr>");
            for (j, c) in row.iter().enumerate() {
                let c = format!(
                    "#{:02x}{:02x}{:02x}",
                    ((c >> 4) & 3) * 80,
                    ((c >> 2) & 3) * 80,
                    ((c >> 0) & 3) * 80
                );
                eprint!(
                    r#"<td bgcolor="{}" onClick="alert('{:?}')"></td>"#,
                    c,
                    (minx + j as i64, miny + i as i64),
                );
            }
            eprintln!("</tr>");
            eprintln!();
        }

        eprintln!("</table>");
        eprintln!("<br><br>");
    }
}

fn parse_functions() -> Result<Dictionary> {
    let mut dict = Dictionary::new();

    let mut f = BufReader::new(File::open("galaxy.txt")?);

    loop {
        let mut s = String::new();
        f.read_line(&mut s)?;

        if s == "" {
            break;
        }
        if s.chars().all(|c| c.is_whitespace()) {
            continue;
        }
        if s.starts_with("//") {
            continue;
        }

        if s.contains('=') {
            let mut jt = s.split("=");

            let lhs = parse_expr(jt.next().unwrap());
            let rhs = parse_expr(jt.next().unwrap());

            // while lhs.var().is_none() {
            //     match lhs {
            //         Expr::App(f, x) => {
            //             assert!(x.var().is_some());
            //             rhs = Expr::Lam(x.var().unwrap().to_string(), Box::new(rhs));
            //             lhs = *f;
            //         }
            //         _ => unreachable!(),
            //     }
            // }

            let b = lhs.borrow();
            let b = b.value.borrow();
            if let Some(name) = b.atom() {
                dict.insert(name.to_string(), rhs);
            }
        } else {
            panic!()
        }
    }
    Ok(dict)
}

fn expr_to_bool(e: ExprRef, dict: &Dictionary) -> bool {
    use dsl::*;
    as_num(app(app(e, int(0)), int(1)), dict).expect("expr_to_bool: failed to test bool") == 0
}

fn expr_to_list(e: ExprRef, dict: &Dictionary) -> VecDeque<ExprRef> {
    use dsl::*;

    if expr_to_bool(app(atom("isnil"), Rc::clone(&e)), dict) {
        VecDeque::new()
    } else {
        let car = app(atom("car"), Rc::clone(&e));
        let cdr = app(atom("cdr"), Rc::clone(&e));
        let mut xs = expr_to_list(cdr, dict);
        xs.push_front(car);
        xs
    }
}

fn value_to_list(v: Value) -> VecDeque<Value> {
    match v {
        Value::Nil => VecDeque::new(),
        Value::Cons(a, b) => {
            let mut ret = value_to_list(*b);
            ret.push_front(*a);
            ret
        }
        Value::Int(_) => panic!(),
    }
}

fn expr_to_value(e: ExprRef, dict: &Dictionary) -> Value {
    use dsl::*;

    if expr_to_bool(app(atom("isnil"), Rc::clone(&e)), dict) {
        Value::Nil
    } else {
        let car = app(atom("car"), Rc::clone(&e));
        let cdr = app(atom("cdr"), Rc::clone(&e));
        Value::cons(expr_to_value(car, dict), expr_to_value(cdr, dict))
    }
}

fn value_to_expr(e: &Value) -> ExprRef {
    use dsl::*;

    match e {
        Value::Int(n) => int(*n),
        Value::Nil => atom("nil"),
        Value::Cons(a, b) => app(app(atom("cons"), value_to_expr(a)), value_to_expr(b)),
    }
}

fn interact(dict: &Dictionary, state: ExprRef, event: ExprRef) -> (ExprRef, ExprRef) {
    use dsl::*;

    let expr = app(app(atom("galaxy"), state), event);
    let res = eval(expr, dict);

    let res = expr_to_list(res, dict);
    assert_eq!(res.len(), 3);

    let flag = Rc::clone(&res[0]);
    let new_state = Rc::clone(&res[1]);
    let data = Rc::clone(&res[2]);

    let flag = as_num(flag, dict).unwrap_or_else(|| panic!("interact: flag is not zero"));

    if flag == 0 {
        (new_state, data)
    } else {
        interact(
            dict,
            new_state,
            value_to_expr(&send_request_value(&expr_to_value(data, dict))),
        )
    }
}

fn print_image(images: ExprRef, dict: &Dictionary) {
    // assume e is [[(int, int)]]

    use dsl::*;

    let mut vs = vec![];

    for image in expr_to_list(images, dict) {
        let mut v = vec![];

        let pts = expr_to_list(image, dict);
        for pt in pts {
            let x = as_num(app(atom("car"), Rc::clone(&pt)), dict)
                .expect("images contains non-integer value");
            let y = as_num(app(atom("cdr"), pt), dict).expect("images contains non-integer value");

            v.push((x, y));
        }

        vs.push(v);
    }

    plot(&vs, false);
}

fn run(opt: &RunOpt) -> Result<()> {
    use dsl::*;

    let dict = parse_functions()?;

    let mut state = value_to_expr(&parse_sexp_str(&opt.state).ok_or("Failed to parse state")?);
    let input = value_to_list(parse_sexp_str(&opt.input).ok_or("Failed to parse input")?);

    for (step, pt) in input.iter().enumerate() {
        let pt = value_to_expr(&pt);
        let x = app(atom("car"), Rc::clone(&pt));
        let y = app(atom("cdr"), pt);

        let click = app(app(atom("cons"), Rc::clone(&x)), Rc::clone(&y));
        let (new_state, images) = interact(&dict, state, click);

        println!("step:  {}", step + 1);
        println!(
            "input: {:?}",
            (as_num(x, &dict).unwrap(), as_num(y, &dict).unwrap())
        );
        println!(
            "state: {}",
            expr_to_value(Rc::clone(&new_state), &dict).print()
        );

        print_image(images, &dict);

        state = new_state;
    }

    Ok(())
}

#[derive(StructOpt, Debug)]
struct DemodOpt {
    arg: String,
}

#[derive(StructOpt, Debug)]
struct SendOpt {
    msg: String,
}

#[derive(StructOpt, Debug)]
struct RunOpt {
    state: String,
    input: String,
}

#[derive(StructOpt, Debug)]
enum Opt {
    Send(SendOpt),
    SendRaw,
    Run(RunOpt),
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Send(opt) => {
            let msg = parse_sexp_str(&opt.msg).unwrap();
            let b = encode(&modulate(&msg));
            eprintln!("request:  {} = {}", msg.to_sexp(), &b);
            let b = send_request(&b);
            let v = demodulate(&decode(&b)).unwrap();
            eprintln!("response: {} = {}", v.to_sexp(), b);
            println!("{}", v.to_sexp());
        }
        Opt::SendRaw => loop {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let s = s.trim().to_owned();
            println!("send: {}", &s);
            let resp = send_request(&s);
            println!("resp: {}", resp);
        },
        Opt::Run(opt) => {
            run(&opt)?;
        }
    }

    Ok(())
}
