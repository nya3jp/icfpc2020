#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum Literal {
    Var(String),
    Int(i64),
    Canvas(Vec<(i64, i64)>),
}

impl Literal {
    fn new(s: &str) -> Literal {
        if let Ok(n) = s.parse() {
            Literal::Int(n)
        } else {
            Literal::Var(s.to_string())
        }
    }

    pub fn print(&self) -> String {
        match self {
            Literal::Int(n) => format!("{}", n),
            Literal::Var(s) => format!("{}", s),
            Literal::Canvas(ps) => format!("<canvas:{:?}>", ps),
        }
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Lam(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Lit(Literal),
    Builtin(String, usize, Vec<Expr>),
}

#[derive(Debug, Clone)]
enum CExpr {
    App(Box<Expr>, Box<Expr>),
    Lit(Literal),
}

// #[derive(Debug)]
// enum Expr2 {
//     App(Box<Expr2>, Vec<Box<Expr2>>),
//     Lit(Literal),
// }

impl Expr {
    fn make_var(s: &str) -> Expr {
        Expr::Lit(Literal::Var(s.to_owned()))
    }

    fn make_builtin(s: &str, arity: usize) -> Expr {
        Expr::Builtin(s.to_owned(), arity, vec![])
    }

    fn make_int(n: i64) -> Expr {
        Expr::Lit(Literal::Int(n))
    }

    fn make_app(f: Expr, x: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(x))
    }

    fn make_apps(f: Expr, xs: Vec<Expr>) -> Expr {
        let mut ret = f;
        for x in xs {
            ret = Expr::make_app(ret, x);
        }
        ret
    }

    fn var(&self) -> Option<&str> {
        match self {
            Expr::Lit(lit) => match lit {
                Literal::Var(v) => Some(&v),
                _ => None,
            },
            _ => None,
        }
    }

    fn int(&self) -> Option<i64> {
        match self {
            Expr::Lit(lit) => match lit {
                Literal::Int(n) => Some(*n),
                _ => None,
            },
            _ => None,
        }
    }

    // fn refine(&self) -> Expr2 {
    //     match self {
    //         Expr::App(f, x) => {
    //             let mut args = vec![Box::new(x.refine())];
    //             let mut f = f.clone();
    //             loop {
    //                 match *f {
    //                     Expr::App(g, y) => {
    //                         f = g.clone();
    //                         args.push(Box::new(y.refine()));
    //                     }
    //                     Expr::Lit(s) => {
    //                         args.reverse();
    //                         return Expr2::App(Box::new(Expr2::Lit(s)), args);
    //                     }
    //                 }
    //             }
    //         }
    //         Expr::Lit(s) => Expr2::Lit(s.clone()),
    //     }
    // }

    fn print(&self) -> String {
        match self {
            Expr::Lam(var, x) => format!("\\{} -> {}", var, x.print()),
            Expr::App(f, x) => format!("({} {})", f.print(), x.print()),
            Expr::Lit(lit) => lit.print(),
            Expr::Builtin(f, _, xs) => format!("<builtin:{}:{}>", f, xs.len()),
        }
    }

    fn to_scheme(&self) -> String {
        match self {
            Expr::Lam(var, x) => format!("\\{} -> {}", var, x.to_scheme()),
            Expr::App(f, x) => format!("({} {})", f.to_scheme(), x.to_scheme()),
            Expr::Lit(lit) => match lit {
                Literal::Int(n) => format!("{}", n),
                Literal::Var(v) => format!("({})", v),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn cexpr(&self) -> Expr {
        match self {
            Expr::Lit(lit) => Expr::Lit(lit.clone()),
            Expr::App(f, x) => Expr::make_app(f.cexpr(), x.cexpr()),
            Expr::Lam(var, x) => {
                if !x.contains(var) {
                    Expr::make_app(Expr::make_var("k"), x.cexpr())
                } else if x.var() == Some(&var) {
                    Expr::make_var("i")
                } else {
                    match x.as_ref() {
                        Expr::Lam(_, _) => Expr::Lam(var.to_string(), Box::new(x.cexpr())).cexpr(),
                        Expr::App(f, x) => {
                            let fc = f.contains(var);
                            let xc = x.contains(var);

                            if fc && xc {
                                Expr::make_app(
                                    Expr::make_app(
                                        Expr::make_var("s"),
                                        Expr::Lam(var.to_string(), f.clone()).cexpr(),
                                    ),
                                    Expr::Lam(var.to_string(), x.clone()).cexpr(),
                                )
                            } else if fc {
                                Expr::make_app(
                                    Expr::make_app(
                                        Expr::make_var("c"),
                                        Expr::Lam(var.to_string(), f.clone()).cexpr(),
                                    ),
                                    x.cexpr(),
                                )
                            } else if xc {
                                Expr::make_app(
                                    Expr::make_app(Expr::make_var("b"), f.cexpr()),
                                    Expr::Lam(var.to_string(), x.clone()).cexpr(),
                                )
                            } else {
                                unreachable!()
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn contains(&self, var: &str) -> bool {
        match self {
            Expr::Lit(lit) => match lit {
                Literal::Var(v) => v == var,
                _ => false,
            },
            Expr::App(f, x) => f.contains(var) || x.contains(var),
            Expr::Lam(v, x) => {
                if v == var {
                    false
                } else {
                    x.contains(var)
                }
            }
            _ => unreachable!(),
        }
    }

    fn eval(&self, dict: &BTreeMap<String, Expr>) -> Expr {
        // eprintln!("{:?}", self.print());
        match self {
            Expr::Lam(_, _) => unreachable!(),
            Expr::App(f, x) => {
                let f = f.eval(dict);
                match f {
                    Expr::Builtin(f, arity, mut xs) => {
                        xs.push(x.as_ref().clone());
                        if xs.len() == arity {
                            eval_builtin(&f, &xs, dict)
                        } else {
                            Expr::Builtin(f, arity, xs)
                        }
                    }
                    _ => unreachable!("Invalid app: {:?} {:?}", f.print(), x.print()),
                }
            }
            Expr::Builtin(_, _, _) => self.clone(),
            Expr::Lit(lit) => match lit {
                Literal::Int(_n) => self.clone(),
                Literal::Var(var) => {
                    if let Some(e) = dict.get(var.as_str()) {
                        e.eval(dict)
                    } else {
                        panic!("variable: {} is not defined", var)
                    }
                }
                Literal::Canvas(_) => self.clone(),
            },
        }
    }

    fn to_value(&self, dict: &BTreeMap<String, Expr>) -> Value {
        if let Some(n) = self.int() {
            Value::int(n)
        } else {
            match self {
                Expr::Lit(Literal::Canvas(ps)) => Value::Canvas(ps.clone()),
                _ => {
                    let b = Expr::make_apps(
                        Expr::make_var("isnil"),
                        vec![self.clone(), Expr::make_int(1), Expr::make_int(0)],
                    )
                    .eval(dict);

                    let b = b.int().unwrap();

                    if b == 1 {
                        Value::Nil
                    } else {
                        let car = Expr::make_app(Expr::make_var("car"), self.clone()).eval(dict);
                        let cdr = Expr::make_app(Expr::make_var("cdr"), self.clone()).eval(dict);
                        Value::cons(car.to_value(dict), cdr.to_value(dict))
                    }
                }
            }
        }
    }
}

fn eval_builtin(f: &str, xs: &[Expr], dict: &BTreeMap<String, Expr>) -> Expr {
    match f {
        "s" => {
            let x0 = xs[0].clone();
            let x1 = xs[1].clone();
            let x2 = xs[2].clone();
            Expr::make_app(Expr::make_app(x0, x2.clone()), Expr::make_app(x1, x2)).eval(dict)
        }
        "k" => xs[0].eval(dict),
        "i" => xs[0].eval(dict),

        "b" => {
            let x0 = xs[0].clone();
            let x1 = xs[1].clone();
            let x2 = xs[2].clone();
            Expr::make_app(x0, Expr::make_app(x1, x2)).eval(dict)
        }
        "c" => {
            let x0 = xs[0].clone();
            let x1 = xs[1].clone();
            let x2 = xs[2].clone();
            Expr::make_app(Expr::make_app(x0, x2), x1).eval(dict)
        }

        "add" => {
            let x = xs[0].eval(dict).int().unwrap();
            let y = xs[1].eval(dict).int().unwrap();
            Expr::make_int(x + y)
        }
        "mul" => {
            let x = xs[0].eval(dict).int().unwrap();
            let y = xs[1].eval(dict).int().unwrap();
            Expr::make_int(x * y)
        }
        "div" => {
            let x = xs[0].eval(dict).int().unwrap();
            let y = xs[1].eval(dict).int().unwrap();
            Expr::make_int(x / y)
        }
        "neg" => {
            let x = xs[0].eval(dict).int().unwrap();
            Expr::make_int(-x)
        }
        "eq" => {
            let x = xs[0].eval(dict).int().unwrap();
            let y = xs[1].eval(dict).int().unwrap();
            dict.get(if x == y { "t" } else { "f" }).unwrap().eval(dict)
        }
        "lt" => {
            let x = xs[0].eval(dict).int().unwrap();
            let y = xs[1].eval(dict).int().unwrap();
            dict.get(if x < y { "t" } else { "f" }).unwrap().eval(dict)
        }

        "ifzero" => {
            let b = xs[0].eval(dict).int().unwrap();
            if b == 0 {
                xs[1].eval(dict)
            } else {
                xs[2].eval(dict)
            }
        }

        "draw" => {
            let xs = xs[0].eval(dict);
            let mut xs = xs.to_value(dict);
            let mut ps = vec![];

            loop {
                match xs {
                    Value::Nil => {
                        break;
                    }
                    Value::Cons(hd, tail) => match *hd {
                        Value::Cons(x, y) => match (*x, *y) {
                            (Value::Int(x), Value::Int(y)) => {
                                ps.push((x, y));
                                xs = *tail;
                            }
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }

            Expr::Lit(Literal::Canvas(ps))
        }

        "send" => todo!(),

        _ => unreachable!(),
    }
}

// impl Expr2 {
//     fn print(&self) -> String {
//         match self {
//             Expr2::App(f, args) => {
//                 let mut s = format!("({}", f.print());
//                 for a in args {
//                     s += &format!(" {}", a.print());
//                 }
//                 s += ")";
//                 s
//             }
//             Expr2::Lit(s) => s.print(),
//         }
//     }

//     fn pp(&self) -> String {
//         self.print()
//         // let mut s = self.print();
//         // if s.chars().last() == Some(')') {
//         //     s.pop();
//         // } else if s.chars().next() == Some('(') {
//         //     s = s[1..].to_string()
//         // }
//         // s
//     }
// }

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Int(i64),
    Nil,
    Cons(Box<Value>, Box<Value>),
    Canvas(Vec<(i64, i64)>),
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
            Value::Canvas(ps) => format!("<canvas:{:?}>", ps),
        }
    }

    fn to_expr(&self) -> Expr {
        match self {
            Value::Int(n) => Expr::make_int(*n),
            Value::Nil => Expr::make_var("nil"),
            Value::Cons(hd, tl) => {
                Expr::make_apps(Expr::make_var("cons"), vec![hd.to_expr(), tl.to_expr()])
            }
            Value::Canvas(ps) => Expr::Lit(Literal::Canvas(ps.clone())),
        }
    }

    fn to_sexp(&self) -> String {
        match self {
            &Value::Int(n) => format!("{}", n),
            Value::Nil => "()".to_string(),
            Value::Cons(hd, tl) => format!("({} . {})", hd.to_sexp(), tl.to_sexp()),
            _ => unreachable!(),
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

fn parse<'a>(it: &mut impl Iterator<Item = &'a str>) -> Expr {
    let s = it.next().unwrap();
    if s == "ap" {
        let f = parse(it);
        let x = parse(it);
        Expr::App(Box::new(f), Box::new(x))
    } else if s == "(" {
        let mut v = vec![];
        loop {
            v.push(parse(it));
            let s = it.next().unwrap();
            if s == "," {
                continue;
            } else if s == ")" {
                let mut ret = Expr::make_var("nil");

                for x in v.into_iter().rev() {
                    ret = Expr::make_app(Expr::make_app(Expr::make_var("cons"), x), ret);
                }

                break ret;
            } else {
                unreachable!();
            }
        }
    } else {
        Expr::Lit(Literal::new(s))
    }
}

fn parse_expr(s: &str) -> Expr {
    let mut it = s.split_whitespace();
    let ret = parse(&mut it);
    assert!(it.next().is_none());
    ret
}

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct DemodOpt {
    arg: String,
}

#[derive(StructOpt, Debug)]
struct SendOpt {
    msg: String,
}

#[derive(StructOpt, Debug)]
struct ParseOpt {}

#[derive(StructOpt, Debug)]
enum Opt {
    Send(SendOpt),
    Parse(ParseOpt),
    // Demod(DemodOpt),
}

use std::iter::Peekable;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Opt::Parse(_opt) => {
            let mut dict = BTreeMap::new();

            let builtins = [
                ("s", 3),
                ("k", 2),
                ("i", 1),
                ("b", 3),
                ("c", 3),
                ("add", 2),
                ("mul", 2),
                ("div", 2),
                ("neg", 1),
                ("eq", 2),
                ("lt", 2),
                ("ifzero", 3),
                ("draw", 1),
                ("send", 1),
            ];

            for (name, arity) in builtins.iter() {
                dict.insert(name.to_string(), Expr::make_builtin(name, *arity));
            }

            loop {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s)?;
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

                    let mut lhs = parse_expr(jt.next().unwrap());
                    let mut rhs = parse_expr(jt.next().unwrap());

                    while lhs.var().is_none() {
                        match lhs {
                            Expr::App(f, x) => {
                                assert!(x.var().is_some());
                                rhs = Expr::Lam(x.var().unwrap().to_string(), Box::new(rhs));
                                lhs = *f;
                            }
                            _ => unreachable!(),
                        }
                    }

                    dbg!(&rhs.print());

                    dict.insert(lhs.var().unwrap().to_string(), rhs.cexpr());
                } else {
                    // println!("{}", parse_expr(&s).refine().pp());
                    panic!()
                }
            }

            // println!(r#"(load "lib.scm")"#);

            for (var, expr) in dict.iter() {
                // println!("(define ({}) {})", var, expr.print());
                println!("{} = {}", var, expr.print());
            }

            let state = Expr::make_var("nil");

            let state = parse_sexp_str("(0 . ((0 . ()) . (0 . (() . ()))))")
                .unwrap()
                .to_expr();

            let expr = Expr::make_app(
                Expr::make_app(
                    Expr::make_app(Expr::make_var("interact"), Expr::make_var("galaxy")),
                    state,
                ),
                Expr::make_app(
                    Expr::make_app(Expr::make_var("vec"), Expr::make_int(0)),
                    Expr::make_int(0),
                ),
            );

            let t = expr.eval(&dict);
            eprintln!("{}", t.to_value(&dict).print());
        }
    }

    Ok(())

    // println!("{}", demodulate(&decode("1101000")).unwrap().print());

    // return Ok(());

    // let v = Value::list(vec![Value::list(vec![Value::cons(
    //     Value::int(1),
    //     Value::int(0),
    // )])]);

    // println!("{}", encode(&modulate(&v)));
}
