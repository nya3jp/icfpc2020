#![allow(dead_code)]
#[derive(Debug, Clone)]
enum Literal {
    Var(String),
    Int(i64),
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
        }
    }
}

#[derive(Debug, Clone)]
enum Expr {
    App(Box<Expr>, Box<Expr>),
    Lit(Literal),
}

#[derive(Debug)]
enum Expr2 {
    App(Box<Expr2>, Vec<Box<Expr2>>),
    Lit(Literal),
}

impl Expr {
    fn new_var(s: &str) -> Expr {
        Expr::Lit(Literal::Var(s.to_owned()))
    }

    fn new_app(f: Expr, x: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(x))
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

    fn refine(&self) -> Expr2 {
        match self {
            Expr::App(f, x) => {
                let mut args = vec![Box::new(x.refine())];
                let mut f = f.clone();
                loop {
                    match *f {
                        Expr::App(g, y) => {
                            f = g.clone();
                            args.push(Box::new(y.refine()));
                        }
                        Expr::Lit(s) => {
                            args.reverse();
                            return Expr2::App(Box::new(Expr2::Lit(s)), args);
                        }
                    }
                }
            }
            Expr::Lit(s) => Expr2::Lit(s.clone()),
        }
    }

    fn print(&self) -> String {
        match self {
            Expr::App(f, x) => format!("({} {})", f.print(), x.print()),
            Expr::Lit(lit) => match lit {
                Literal::Int(n) => format!("{}", n),
                Literal::Var(v) => format!("({})", v),
            },
        }
    }
}

impl Expr2 {
    fn print(&self) -> String {
        match self {
            Expr2::App(f, args) => {
                let mut s = format!("({}", f.print());
                for a in args {
                    s += &format!(" {}", a.print());
                }
                s += ")";
                s
            }
            Expr2::Lit(s) => s.print(),
        }
    }

    fn pp(&self) -> String {
        self.print()
        // let mut s = self.print();
        // if s.chars().last() == Some(')') {
        //     s.pop();
        // } else if s.chars().next() == Some('(') {
        //     s = s[1..].to_string()
        // }
        // s
    }
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
                hd.modulate(v);
                tl.modulate(v);
            }
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

    fn print(&self) -> String {
        match self {
            &Value::Int(n) => format!("{}", n),
            Value::Nil => "nil".to_string(),
            Value::Cons(hd, tl) => format!("({}, {})", hd.print(), tl.print()),
        }
    }

    fn to_sexp(&self) -> String {
        match self {
            &Value::Int(n) => format!("{}", n),
            Value::Nil => "()".to_string(),
            Value::Cons(hd, tl) => format!("({} . {})", hd.to_sexp(), tl.to_sexp()),
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
                let mut ret = Expr::new_var("nil");

                for x in v.into_iter().rev() {
                    ret = Expr::new_app(Expr::new_app(Expr::new_var("cons"), x), ret);
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
            Ok(())
        }
        Opt::Parse(_opt) => {
            let mut defs = vec![];

            loop {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s)?;
                if s == "" {
                    break;
                }

                if s.contains('=') {
                    let mut jt = s.split("=");

                    let lhs = parse_expr(jt.next().unwrap());
                    let rhs = parse_expr(jt.next().unwrap());

                    if lhs.var().is_none() {
                        println!("{} = {}", lhs.print(), rhs.print());
                    } else {
                        defs.push((lhs.var().unwrap().to_owned(), rhs));
                    }

                // println!("{} = {}", lhs.refine().pp(), rhs.refine().pp());
                } else {
                    // println!("{}", parse_expr(&s).refine().pp());
                    panic!()
                }
            }

            println!(r#"(load "lib.scm")"#);

            for (var, expr) in defs {
                println!("(define ({}) {})", var, expr.print());
            }

            Ok(())
        }
    }

    // println!("{}", demodulate(&decode("1101000")).unwrap().print());

    // return Ok(());

    // let v = Value::list(vec![Value::list(vec![Value::cons(
    //     Value::int(1),
    //     Value::int(0),
    // )])]);

    // println!("{}", encode(&modulate(&v)));
    // return Ok(());
}
