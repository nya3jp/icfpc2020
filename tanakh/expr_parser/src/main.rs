#[derive(Debug, Clone)]
enum Literal {
    Unknown(i64),
    Var(String),
    Int(i64),
}

impl Literal {
    fn new(s: &str) -> Literal {
        if s.chars().next().unwrap() == ':' {
            Literal::Unknown(s[1..].parse().unwrap())
        } else if s.chars().all(|c| c.is_ascii_digit()) {
            Literal::Int(s.parse().unwrap())
        } else {
            Literal::Var(s.to_string())
        }
    }

    fn print(&self) -> String {
        match self {
            Literal::Int(n) => format!("{}", n),
            Literal::Unknown(n) => format!(":{}", n),
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

#[derive(Debug)]
enum Value {
    Int(i64),
    Nil,
    Cons(Box<Value>, Box<Value>),
}

impl Value {
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
enum Opt {
    Demod(DemodOpt),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // match Opt::from_args() {
    //     Opt::Demod(s) => {
    //         let v = s.arg.chars().map(|c| c == '1').collect::<Vec<_>>();
    //         println!("{}", demodulate(&v).unwrap().print());
    //     }
    // }

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;
        if s == "" {
            break;
        }

        if s.contains('=') {
            let mut jt = s.split("=");
            let mut first = true;

            while let Some(s) = jt.next() {
                if first {
                    first = false;
                } else {
                    print!(" = ");
                }
                print!("{}", parse_expr(s).refine().pp());
            }
            println!();
        } else {
            println!("{}", parse_expr(&s).refine().pp());
        }
    }

    Ok(())
}
