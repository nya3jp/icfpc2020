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

use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use super::simplified;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Int(isize),
    Symbol(String),
    List(Vec<Expr>),
    True,
    False,
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Equ(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    IsNil(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    // match val {
    //   [] => e1,
    //   hd::tl => e2,
    // }
    MatchCons(Box<Expr>, Box<Expr>, String, String, Box<Expr>),
    // let hd::tl = e1 in e2
    LetCons(String, String, Box<Expr>, Box<Expr>),

    // let x = e1 in e2
    Let(String, Box<Expr>, Box<Expr>),

    Call(Vec<Expr>),

    // For partial application / func call.
    ADD,
    CAR,
    CDR,
    CONS,
    DIV,
    EQ,
    ISNIL,
    LT,
    MUL,
    NEG,
    Func(Vec<String>, Box<Expr>),
}

impl Expr {
    fn priority(&self) -> usize {
        match self {
            Expr::Int(_) | Expr::Symbol(_) | Expr::List(_) | Expr::True | Expr::False |
            Expr::ADD | Expr::CAR | Expr::CDR | Expr::CONS | Expr::DIV |
            Expr::EQ | Expr::ISNIL | Expr::LT | Expr::MUL | Expr::NEG => 0,
            Expr::Neg(_) | Expr::IsNil(_) | Expr::Not(_) => 1,
            Expr::Add(_, _) | Expr::Sub(_, _) => 4,
            Expr::Mul(_, _) | Expr::Div(_, _) | Expr::Mod(_, _) => 3,
            Expr::Equ(_, _) | Expr::Lt(_, _) => 5,
            Expr::And(_, _) => 6,
            Expr::Or(_, _) => 7,
            Expr::If(_, _, _) => 8,
            Expr::MatchCons(_, _, _, _, _) | Expr::LetCons(_, _, _, _) | Expr::Let(_, _, _) => 9,
            Expr::Call(_) => 1,
            Expr::Func(_, _) => 2,
        }
    }

    fn name(&self) -> Option<&str> {
        match self {
            Expr::Symbol(name) => Some(name),
            _ => None,
        }
    }

    fn fmt_with_paren(&self, pri: usize, f: &mut fmt::Formatter) -> fmt::Result {
        let self_pri = self.priority();
        if self_pri > pri {
            write!(f, "(")?;
        }
        self.fmt(f)?;
        if self_pri > pri {
            write!(f, ")")?;
        }
        Ok(())
    }

    fn fmt_list(iter: &mut dyn Iterator<Item = &Expr>, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for ref e in iter {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", e)?;
        }
        Ok(())
    }

    fn fmt_params(iter: &mut dyn Iterator<Item = &String>, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for ref e in iter {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", e)?;
        }
        Ok(())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pri = self.priority();
        match self {
            Expr::Int(i) => write!(f, "{}", i),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::List(lst) => {
                write!(f, "[")?;
                Expr::fmt_list(&mut lst.iter(), f)?;
                write!(f, "]")
            },
            Expr::ADD => write!(f, "add"),
            Expr::CAR => write!(f, "car"),
            Expr::CDR => write!(f, "cdr"),
            Expr::CONS => write!(f, "cons"),
            Expr::DIV => write!(f, "div"),
            Expr::EQ => write!(f, "=="),
            Expr::ISNIL => write!(f, "isnil"),
            Expr::LT => write!(f, "<"),
            Expr::MUL => write!(f, "mul"),
            Expr::NEG => write!(f, "neg"),
            Expr::True => write!(f, "true"),
            Expr::False => write!(f, "false"),
            Expr::Neg(e) => {
                write!(f, "-")?;
                e.fmt_with_paren(pri, f)
            },
            Expr::IsNil(e) => write!(f, "isnil({})", e),
            Expr::Add(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " + ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Sub(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " - ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Mul(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " * ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Div(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " / ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Mod(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " % ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Equ(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " == ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Lt(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " < ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Not(e) => {
                write!(f, "!")?;
                e.fmt_with_paren(pri, f)
            },
            Expr::And(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " && ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::Or(lhs, rhs) => {
                lhs.fmt_with_paren(pri, f)?;
                write!(f, " || ")?;
                rhs.fmt_with_paren(pri-1, f)
            },
            Expr::If(cond, e1, e2) =>
                write!(f, "if {} {{ {} }} else {{ {} }}", cond, e1, e2),
            Expr::MatchCons(val, e1, hd, tl, e2) =>
                write!(f, "match {} {{ [] => {}, {}::{} => {} }}", val, e1, hd, tl, e2),
            Expr::LetCons(hd, tl, e1, e2) =>
                write!(f, "let {}::{} = {} in {}", hd, tl, e1, e2),
            Expr::Let(var, e1, e2) =>
                write!(f, "let {} = {} in {}", var, e1, e2),
            Expr::Call(vals) => {
                let mut iter = vals.iter();
                let fun = iter.next().unwrap();
                fun.fmt_with_paren(1, f)?;
                write!(f, "(")?;
                Expr::fmt_list(&mut iter, f)?;
                write!(f, ")")
            },
            Expr::Func(params, body) => {
                write!(f, "fun(")?;
                Expr::fmt_params(&mut params.iter(), f)?;
                write!(f, ") -> {}", body)
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Int,
    Bool,
    List,
    Func(Vec<(Option<String>, Option<Type>)>, Box<Option<Type>>),
}

pub struct Env {
    env: HashMap<String, (String, Option<Type>)>,
}

impl Env {
    pub fn new() -> Self {
        Self{env: HashMap::new()}
    }

    pub fn insert(&mut self, orig_name: &str, name: &str, t: Option<Type>) {
        self.env.insert(orig_name.to_string(), (name.to_string(), t));
    }

    pub fn get_name(&self, name: &str) -> Option<&String> {
        if let Some((n, _)) = self.env.get(name) {
            Some(n)
        } else {
            None
        }
    }

    pub fn get_type(&self, name: &str) -> Option<&Type> {
        if let Some((_, t)) = self.env.get(name) {
            match t {
                Some(t) => Some(t),
                None => None
            }
        } else {
            None
        }
    }
}

struct Constructor<'a> {
    type_env: HashMap<String, Type>,
    env: &'a Env,
}

impl<'a> Constructor<'a> {
    fn new(env: &'a Env) -> Self {
        Self{type_env: HashMap::new(), env}
    }

    fn construct(&mut self, val: &simplified::Value, _: Option<&Type>) -> Expr {
        match val {
            simplified::Value::SYMBOL(s) => Expr::Symbol(s.clone()),
            simplified::Value::INT(i) => Expr::Int(*i),
            simplified::Value::T => Expr::True,
            simplified::Value::F => Expr::False,
            simplified::Value::ADD => Expr::ADD,
            simplified::Value::CAR => Expr::CAR,
            simplified::Value::CDR => Expr::CDR,
            simplified::Value::CONS => Expr::CONS,
            simplified::Value::DIV => Expr::DIV,
            simplified::Value::EQ => Expr::EQ,
            simplified::Value::ISNIL => Expr::ISNIL,
            simplified::Value::LT => Expr::LT,
            simplified::Value::MUL => Expr::MUL,
            simplified::Value::NEG => Expr::NEG,
            simplified::Value::Apply(fun, arg) => match self.construct(&*fun, None) {
                Expr::NEG => Expr::Neg(Box::new(self.construct(&*arg, None))),
                e @ Expr::Symbol(_) if self.is_list(&e) => {
                    match (e, self.construct(&*arg, None)) {
                        (Expr::Symbol(s), Expr::Func(params, body)) if params.len() >= 2 => {
                            let (hd, tl, params) = {
                                let mut iter = params.into_iter();
                                let hd = iter.next().unwrap();
                                let tl = iter.next().unwrap();
                                (hd, tl, iter.collect::<Vec<_>>())
                            };
                            let body =
                                if params.len() > 0 {
                                    Box::new(Expr::Func(params.to_vec(), body))
                                } else {
                                    body
                                };
                            Expr::LetCons(hd.to_string(), tl.to_string(), Box::new(Expr::Symbol(s)), body)
                        },
                        (e, arg) => Expr::Call(vec!(e, arg)),
                    }
                },
                Expr::ISNIL => {
                    let e = self.construct(&*arg, None);
                    if let Expr::Symbol(s) = &e {
                        self.type_env.insert(s.clone(), Type::List);
                    }
                    Expr::IsNil(Box::new(e))
                },
                Expr::Let(var, e1, e2) if self.is_or(&var, &*e2) =>
                    Expr::Or(e1, Box::new(self.construct(&*arg, None))),
                Expr::Let(var, e1, e2) if self.is_and(&var, &*e2) =>
                    Expr::And(e1, Box::new(self.construct(&*arg, None))),
                Expr::Call(vals) if vals.len() == 2 => {
                    let (fun, arg0) = {
                        let mut iter = vals.into_iter();
                        let fun = iter.next().unwrap();
                        let arg0 = iter.next().unwrap();
                        (fun, arg0)
                    };
                    let arg1 = self.construct(&*arg, None);
                    match fun {
                        fun if self.is_bool(&fun) => Expr::If(Box::new(fun), Box::new(arg0), Box::new(arg1)),
                        Expr::ADD => Expr::Add(Box::new(arg0), Box::new(arg1)),
                        Expr::MUL => Expr::Mul(Box::new(arg0), Box::new(arg1)),
                        Expr::DIV => Expr::Div(Box::new(arg0), Box::new(arg1)),
                        Expr::EQ => Expr::Equ(Box::new(arg0), Box::new(arg1)),
                        Expr::LT => Expr::Lt(Box::new(arg0), Box::new(arg1)),
                        fun => Expr::Call(vec!(fun, arg0, arg1)),
                    }
                },
                Expr::Call(mut vals) if vals.len() > 2 => {
                    let arg1 = self.construct(&*arg, None);
                    if let Expr::Symbol(s) = &vals[0] {
                        if let Some(Type::Func(ps, r)) = self.get_type(s) {
                            if ps.len() == vals.len() - 2 && **r == Some(Type::Bool) {
                                let arg0 = vals.pop().unwrap();
                                return Expr::If(Box::new(Expr::Call(vals)), Box::new(arg0), Box::new(arg1));
                            }
                        }
                    }
                    vals.push(arg1);
                    Expr::Call(vals)
                },
                Expr::Call(mut vals) => {
                    vals.push(self.construct(&*arg, None));
                    Expr::Call(vals)
                },
                e if self.is_func(&e) => {
                    let (mut ls, ps, body) = self.decompose_let(e);
                    let (var, ps) = {
                        let mut iter = ps.into_iter();
                        let var = iter.next().unwrap();
                        let ps = iter.collect::<Vec<_>>();
                        (var, ps)
                    };
                    ls.push((var, self.construct(&*arg, None)));
                    self.compose_let(ls, ps, body)
                }
                e => Expr::Call(vec!(e, self.construct(&*arg, None))),
            },
            simplified::Value::Func(param, body) => match self.construct(&*body, None) {
                Expr::Func(mut params, body) => {
                    let mut ps = vec!(param.clone());
                    ps.append(&mut params);
                    if self.is_not(&ps, &*body) {
                        let cond = match *body {
                            Expr::If(cond, _, _) => cond,
                            _ => panic!("Unexpected!"),
                        };
                        Expr::Not(cond)
                    } else {
                        Expr::Func(ps, body)
                    }
                },
                e => Expr::Func(vec!(param.clone()), Box::new(e)),
            },
            simplified::Value::List(lst) => Expr::List(lst.iter().map(|v| self.construct(v, None)).collect()),
        }
    }

    fn is_bool(&self, e: &Expr) -> bool {
        match e {
            Expr::True | Expr::False | Expr::Equ(_, _) | Expr::Lt(_, _) |
            Expr::Or(_, _) | Expr::And(_, _) | Expr::Not(_) |
            Expr::IsNil(_) => true,
            Expr::Symbol(s) => self.get_type(s) == Some(&Type::Bool),
            _ => false,
        }
    }

    fn is_list(&self, e: &Expr) -> bool {
        match e {
            Expr::Symbol(s) => self.get_type(s) == Some(&Type::List),
            _ => false,
        }
    }

    fn is_or(&self, name: &str, e: &Expr) -> bool {
        match e {
            Expr::Call(vals) =>
                vals.len() == 2 &&
                vals.iter().all(|x| x.name() == Some(name)),
            _ => false,
        }
    }

    fn is_and(&self, name: &str, e: &Expr) -> bool {
        match e {
            Expr::Func(params, body) if params.len() == 1 => match &**body {
                Expr::Call(vals) if vals.len() == 3 => {
                    let p1 = &name;
                    let p2 = &params[0];
                    let v1 = vals[0].name();
                    let v2 = vals[1].name();
                    let v3 = vals[2].name();
                    (v1 == Some(p1) && v2 == Some(p2) && v3 == Some(p1)) ||
                        (v1 == Some(p2) && v2 == Some(p1) && v3 == Some(p2))
                },
                _ => false,
            },
            _ => false,
        }
    }

    fn is_not(&self, params: &[String], e: &Expr) -> bool {
        eprintln!("!!is_not {:?}, {:?}", params, e);
        if params.len() != 2 {
            return false;
        }
        let p1 = &params[0];
        let p2 = &params[1];
        match e {
            Expr::If(cond, e1, e2) =>
                e1.name() == Some(p2) && e2.name() == Some(p1) &&
                !self.contains_name(p1, cond) && !self.contains_name(p2, cond),
            _ => false,
        }
    }

    fn contains_name(&self, name: &str, e: &Expr) -> bool {
        match e {
            Expr::Symbol(s) => s == name,
            Expr::List(v) | Expr::Call(v) => v.iter().any(|e| self.contains_name(name, e)),
            Expr::Neg(e) | Expr::Not(e) | Expr::IsNil(e) | Expr::Func(_, e) => self.contains_name(name, &**e),
            Expr::Add(e1, e2) | Expr::Sub(e1, e2) | Expr::Mul(e1, e2) | Expr::Div(e1, e2) | Expr::Mod(e1, e2) |
            Expr::Equ(e1, e2) | Expr::Lt(e1, e2) | Expr::Or(e1, e2) | Expr::And(e1, e2) |
            Expr::LetCons(_, _, e1, e2) | Expr::Let(_, e1, e2) =>
                self.contains_name(name, &**e1) || self.contains_name(name, &**e2),
            Expr::MatchCons(e1, e2, _, _, e3) |
            Expr::If(e1, e2, e3) =>
                self.contains_name(name, &**e1) || self.contains_name(name, &**e2) || self.contains_name(name, &**e3),
            _ => false,
        }
    }

    fn get_type(&self, name: &str) -> Option<&Type> {
        if let Some(t) = self.type_env.get(name) {
            return Some(t);
        }
        if let Some(t) = self.env.get_type(name) {
            return Some(t);
        }
        None
    }

    fn is_func(&self, e: &Expr) -> bool {
        match e {
            Expr::Let(_, _, body) => self.is_func(body),
            Expr::Func(_, _) => true,
            _ => false,
        }
    }

    fn decompose_let(&self, e: Expr) -> (Vec<(String, Expr)>, Vec<String>, Expr) {
        match e {
            Expr::Let(var, e1, e2) => {
                let (mut ls, ps, body) = self.decompose_let(*e2);
                ls.push((var, *e1));
                (ls, ps, body)
            },
            Expr::Func(ps, body) => (Vec::new(), ps, *body),
            _ => panic!("unexpected!"),
        }
    }

    fn compose_let(&self, mut ls: Vec<(String, Expr)>, ps: Vec<String>, body: Expr) -> Expr {
        if ls.is_empty() {
            if ps.is_empty() {
                body
            } else {
                Expr::Func(ps, Box::new(body))
            }
        } else {
            let (var, e1) = ls.pop().unwrap();
            let e2 = self.compose_let(ls, ps, body);
            Expr::Let(var, Box::new(e1), Box::new(e2))
        }
    }
}

pub fn construct(val: &simplified::Value, env: &Env) -> Expr {
    Constructor::new(env).construct(val, None)
}

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::List(lst) => Expr::List(lst.iter().map(simplify).collect()),
        Expr::Neg(e) => match simplify(&**e) {
            Expr::Int(i) => Expr::Int(-i),
            e => Expr::Neg(Box::new(e)),
        },
        Expr::Add(lhs, rhs) => {
            let lhs = simplify(&**lhs);
            let rhs = simplify(&**rhs);
            match rhs {
                Expr::Neg(e) => match *e {
                    Expr::Mul(e1, e2) => match *e2 {
                        Expr::Div(e21, e22) if *e21 == lhs && *e22 == *e1 =>
                            Expr::Mod(Box::new(lhs), e1),
                        e2 => Expr::Sub(Box::new(lhs), Box::new(Expr::Mul(e1, Box::new(e2)))),
                    },
                    e => Expr::Sub(Box::new(lhs), Box::new(e)),
                },
                Expr::Int(i) if i < 0 => Expr::Sub(Box::new(lhs), Box::new(Expr::Int(-i))),
                rhs => match lhs {
                    Expr::Neg(e) => Expr::Sub(Box::new(rhs), e),
                    Expr::Int(i) if i < 0 => Expr::Sub(Box::new(rhs), Box::new(Expr::Int(-i))),
                    lhs => Expr::Add(Box::new(lhs), Box::new(rhs)),
                },
            }
        },
        Expr::Mul(lhs, rhs) => {
            let lhs = simplify(&**lhs);
            let rhs = simplify(&**rhs);
            Expr::Mul(Box::new(lhs), Box::new(rhs))
        },
        Expr::Div(lhs, rhs) => {
            let lhs = simplify(&**lhs);
            let rhs = simplify(&**rhs);
            Expr::Div(Box::new(lhs), Box::new(rhs))
        },
        Expr::Equ(lhs, rhs) => {
            let lhs = simplify(&**lhs);
            let rhs = simplify(&**rhs);
            Expr::Equ(Box::new(lhs), Box::new(rhs))
        }
        Expr::Lt(lhs, rhs) => {
            let lhs = simplify(&**lhs);
            let rhs = simplify(&**rhs);
            Expr::Lt(Box::new(lhs), Box::new(rhs))
        }
        Expr::Not(e) =>
            Expr::Not(Box::new(simplify(&**e))),
        Expr::Or(lhs, rhs) => {
            let lhs = simplify(&**lhs);
            let rhs = simplify(&**rhs);
            Expr::Or(Box::new(lhs), Box::new(rhs))
        }
        Expr::And(lhs, rhs) => {
            let lhs = simplify(&**lhs);
            let rhs = simplify(&**rhs);
            Expr::And(Box::new(lhs), Box::new(rhs))
        }
        Expr::IsNil(e) => Expr::IsNil(Box::new(simplify(e))),
        Expr::If(cond, e1, e2) => {
            let cond = simplify(&**cond);
            let e1 = simplify(&**e1);
            let e2 = simplify(&**e2);
            match (cond, e2) {
                (Expr::IsNil(x1), Expr::LetCons(hd, tl, x2, body)) if x1 == x2 =>
                    Expr::MatchCons(x1, Box::new(e1), hd, tl, body),
                (cond, e2) => Expr::If(Box::new(cond), Box::new(e1), Box::new(e2))
            }
        },
        Expr::Call(vals) =>
            Expr::Call(vals.iter().map(simplify).collect()),
        Expr::Func(params, body) =>
            Expr::Func(params.to_vec(), Box::new(simplify(&**body))),
        Expr::LetCons(hd, tl, e1, e2) =>
            Expr::LetCons(hd.clone(), tl.clone(), Box::new(simplify(&**e1)), Box::new(simplify(&**e2))),
        Expr::Let(var, e1, e2) =>
            Expr::Let(var.clone(), Box::new(simplify(&**e1)), Box::new(simplify(&**e2))),
        e @ Expr::MatchCons(_, _, _, _, _) => panic!("unexpected! {:?}", e),
        e => e.clone(),
    }
}

struct Renamer<'a> {
    rename_env: HashMap<String, String>,
    next_id: usize,
    env: &'a Env,
}

impl<'a> Renamer<'a> {
    fn new(env: &'a Env) -> Self {
        Self {rename_env: HashMap::new(), next_id: 0, env}
    }

    fn rename(&mut self, name: &str) -> String {
        if name.starts_with(":") {
            if let Some(n) = self.env.get_name(name) {
                return n.clone();
            }
            return name.to_string();
        }
        if let Some(n) = self.rename_env.get(name) {
            return n.clone();
        }

        // New name. Register.
        let id = self.next_id;
        self.next_id += 1;
        let new_name = format!("_x{}", id);
        self.rename_env.insert(name.to_string(), new_name.clone());
        new_name
    }

    fn rename_expr(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Symbol(s) => Expr::Symbol(self.rename(s)),
            Expr::List(lst) =>
                Expr::List(lst.iter().map(|v| self.rename_expr(v)).collect()),
            Expr::Neg(e) => Expr::Neg(Box::new(self.rename_expr(&**e))),
            Expr::Add(lhs, rhs) =>
                Expr::Add(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::Sub(lhs, rhs) =>
                Expr::Sub(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::Mul(lhs, rhs) =>
                Expr::Mul(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::Div(lhs, rhs) =>
                Expr::Div(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::Mod(lhs, rhs) =>
                Expr::Mod(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::Equ(lhs, rhs) =>
                Expr::Equ(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::Lt(lhs, rhs) =>
                Expr::Lt(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::Not(e) => Expr::Not(Box::new(self.rename_expr(&**e))),
            Expr::Or(lhs, rhs) =>
                Expr::Or(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::And(lhs, rhs) =>
                Expr::And(Box::new(self.rename_expr(&**lhs)), Box::new(self.rename_expr(&**rhs))),
            Expr::IsNil(e) => Expr::IsNil(Box::new(self.rename_expr(&**e))),
            Expr::If(cond, e1, e2) =>
                Expr::If(Box::new(self.rename_expr(&**cond)),
                         Box::new(self.rename_expr(&**e1)),
                         Box::new(self.rename_expr(&**e2))),
            Expr::MatchCons(v, e1, hd, tl, e2) =>
                Expr::MatchCons(Box::new(self.rename_expr(&**v)),
                                Box::new(self.rename_expr(&**e1)),
                                self.rename(hd), self.rename(tl),
                                Box::new(self.rename_expr(&**e2))),
            Expr::LetCons(hd, tl, v, e) =>
                Expr::LetCons(self.rename(hd), self.rename(tl),
                              Box::new(self.rename_expr(&**v)),
                              Box::new(self.rename_expr(&**e))),
            Expr::Let(var, v, e) =>
                Expr::Let(self.rename(var),
                          Box::new(self.rename_expr(&**v)),
                          Box::new(self.rename_expr(&**e))),
            Expr::Call(vals) =>
                Expr::Call(vals.iter().map(|v| self.rename_expr(v)).collect()),
            Expr::Func(params, body) =>
                Expr::Func(params.iter().map(|v| self.rename(v)).collect(), Box::new(self.rename_expr(&**body))),
            e => e.clone(),
        }
    }
}

pub fn rename(expr: &Expr, env: &Env) -> Expr {
    Renamer::new(env).rename_expr(expr)
}
