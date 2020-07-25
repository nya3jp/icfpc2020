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
use super::lambda;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Func(String, Box<Value>),
    Apply(Box<Value>, Box<Value>),
    List(Vec<Value>),
    SYMBOL(String),
    INT(isize),
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
    T,
    F,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Func(name, val) => write!(f, "{{\\{}.{}}}", name, *val),
            Value::Apply(fun, arg) => write!(f, "({} {})", *fun, *arg),
            Value::List(lst) => {
                let mut first = true;
                write!(f, "[")?;
                for v in lst.iter() {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            },
            Value::SYMBOL(s) => write!(f, "{}", s),
            Value::INT(i) => write!(f, "{}", i),
            Value::ADD => write!(f, "add"),
            Value::CAR => write!(f, "car"),
            Value::CDR => write!(f, "cdr"),
            Value::CONS => write!(f, "cons"),
            Value::DIV => write!(f, "div"),
            Value::EQ => write!(f, "=="),
            Value::ISNIL => write!(f, "isnil"),
            Value::LT => write!(f, "<"),
            Value::MUL => write!(f, "mul"),
            Value::NEG => write!(f, "~-"),
            Value::T => write!(f, "t"),
            Value::F => write!(f, "f"),
        }
    }
}

// "false" is "\x.\y.((t y) (x y))" || (true, \x.x)
fn is_false1(val: &lambda::Value) -> bool {
    let (param1, body) = match val {
        lambda::Value::Func(param, body) => (param, body),
        _ => return false,
    };
    let (param2, body) = match &**body {
        lambda::Value::Func(param, body) => (param, body),
        _ => return false,
    };
    **body == lambda::Value::Apply(
        Box::new(lambda::Value::Apply(
            Box::new(lambda::Value::T),
            Box::new(lambda::Value::SYMBOL(param2.clone())),
        )),
        Box::new(lambda::Value::Apply(
            Box::new(lambda::Value::SYMBOL(param1.clone())),
            Box::new(lambda::Value::SYMBOL(param2.clone())),
        )),
    )
}

fn is_false2(val: &lambda::Value) -> bool {
    match val {
        lambda::Value::Apply(fun, arg) => {
            if **fun != lambda::Value::T {
                return false;
            }
            match &**arg {
                lambda::Value::Func(param, body) =>
                    **body == lambda::Value::SYMBOL(param.clone()),
                _ => false,
            }
        },
        _ => false,
    }
}

fn is_false(val: &lambda::Value) -> bool {
    is_false1(val) || is_false2(val)
}

pub fn simplify(value: lambda::Value) -> Value {
    match value {
        value if is_false(&value) => Value::F,
        lambda::Value::Func(param, body) => Value::Func(param, Box::new(simplify(*body))),
        lambda::Value::Apply(fun, arg) => {
            let fun = simplify(*fun);
            let arg = simplify(*arg);
            match (fun, arg) {
                (Value::Apply(f, arg0), Value::List(mut vlist)) if *f == Value::CONS => {
                    let mut result = vec!(*arg0);
                    result.append(&mut vlist);
                    Value::List(result)
                },
                (f, a) => Value::Apply(Box::new(f), Box::new(a))
            }
        },
        lambda::Value::SYMBOL(s) => Value::SYMBOL(s),
        lambda::Value::INT(v) => Value::INT(v),
        lambda::Value::ADD => Value::ADD,
        lambda::Value::CAR => Value::CAR,
        lambda::Value::CDR => Value::CDR,
        lambda::Value::CONS => Value::CONS,
        lambda::Value::DIV => Value::DIV,
        lambda::Value::EQ => Value::EQ,
        lambda::Value::ISNIL => Value::ISNIL,
        lambda::Value::LT => Value::LT,
        lambda::Value::MUL => Value::MUL,
        lambda::Value::NEG => Value::NEG,
        lambda::Value::NIL => Value::List(vec!()),
        lambda::Value::T => Value::T,
    }
}