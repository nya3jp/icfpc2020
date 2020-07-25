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

use std::collections::HashMap;
use std::fmt;
use super::ast;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Func(String, Box<Value>),
    Apply(Box<Value>, Box<Value>),
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
    NIL,
    T,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Func(name, val) => write!(f, "{{\\{}.{}}}", name, *val),
            Value::Apply(fun, arg) => write!(f, "({} {})", *fun, *arg),
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
            Value::NIL => write!(f, "nil"),
            Value::T => write!(f, "t"),
        }
    }
}

struct Lamdifier {
    next_id: usize,
}

impl Lamdifier {
    fn new() -> Self {
        Self{next_id: 0}
    }

    fn lamdify(&mut self, value: &ast::Value) -> Value {
        match value {
            ast::Value::Apply(fun, arg) => {
                let fun = self.lamdify(fun);
                let arg = self.lamdify(arg);
                Value::Apply(Box::new(fun), Box::new(arg))
            },
            ast::Value::SYMBOL(s) => Value::SYMBOL(s.clone()),
            ast::Value::INT(i) => Value::INT(*i),
            ast::Value::ADD => Value::ADD,
            ast::Value::CAR => Value::CAR,
            ast::Value::CDR => Value::CDR,
            ast::Value::CONS => Value::CONS,
            ast::Value::DIV => Value::DIV,
            ast::Value::EQ => Value::EQ,
            ast::Value::ISNIL => Value::ISNIL,
            ast::Value::LT => Value::LT,
            ast::Value::MUL => Value::MUL,
            ast::Value::NEG => Value::NEG,
            ast::Value::NIL => Value::NIL,
            ast::Value::T => Value::T,
            ast::Value::I => self.new_i(),
            ast::Value::B => self.new_b(),
            ast::Value::C => self.new_c(),
            ast::Value::S => self.new_s(),
        }
    }

    fn new_id(&mut self) -> String {
        let name = format!("x{}", self.next_id);
        self.next_id += 1;
        name
    }

    // (I x) = x
    fn new_i(&mut self) -> Value {
        let x = self.new_id();
        Value::Func(x.clone(), Box::new(Value::SYMBOL(x)))
    }

    // (B x0 x1 x2) = (x0 (x1 x2))
    fn new_b(&mut self) -> Value {
        let x0 = self.new_id();
        let x1 = self.new_id();
        let x2 = self.new_id();
        Value::Func(x0.clone(), Box::new(
            Value::Func(x1.clone(), Box::new(
                Value::Func(x2.clone(), Box::new(
                    Value::Apply(
                        Box::new(Value::SYMBOL(x0)),
                        Box::new(Value::Apply(
                            Box::new(Value::SYMBOL(x1)),
                            Box::new(Value::SYMBOL(x2))
                        ))
                    )
                ))
            ))
        ))
    }

    // (C x0 x1 x2) = (x0 x2 x1)
    fn new_c(&mut self) -> Value {
        let x0 = self.new_id();
        let x1 = self.new_id();
        let x2 = self.new_id();
        Value::Func(x0.clone(), Box::new(
            Value::Func(x1.clone(), Box::new(
                Value::Func(x2.clone(), Box::new(
                    Value::Apply(
                        Box::new(Value::Apply(
                            Box::new(Value::SYMBOL(x0)),
                            Box::new(Value::SYMBOL(x2)),
                        )),
                        Box::new(Value::SYMBOL(x1)),
                    )
                ))
            ))
        ))
    }

    // (S x0 x1 x2) = ((x0 x2) (x1 x2))
    fn new_s(&mut self) -> Value {
        let x0 = self.new_id();
        let x1 = self.new_id();
        let x2 = self.new_id();
        Value::Func(x0.clone(), Box::new(
            Value::Func(x1.clone(), Box::new(
                Value::Func(x2.clone(), Box::new(
                    Value::Apply(Box::new(Value::Apply(Box::new(Value::SYMBOL(x0)),
                                                       Box::new(Value::SYMBOL(x2.clone())))),
                                 Box::new(Value::Apply(Box::new(Value::SYMBOL(x1)),
                                                       Box::new(Value::SYMBOL(x2)))),
                    )
                ))
            ))
        ))
    }
}

pub fn lamdify(value: &ast::Value) -> Value {
    Lamdifier::new().lamdify(value)
}

fn count_symbols(value: &Value) -> HashMap<String, usize> {
    fn aux(counts: &mut HashMap<String, usize>, value: &Value) {
        match value {
            Value::Func(_, body) => aux(counts, body),
            Value::Apply(func, arg) => {
                aux(counts, func);
                aux(counts, arg);
            },
            Value::SYMBOL(s) => *counts.entry(s.clone()).or_insert(0) += 1,
            _ => (),
        }
    }
    let mut counts = HashMap::new();
    aux(&mut counts, value);
    counts
}

// Takes the alpha-normalized value, and returns its evaluated one.
// Here, substitution happens only when the variable appears only once.
pub fn eval(value: Value) -> Value {
    fn substitute(value: Value, env: &mut HashMap<String, Value>, num_symbols: &HashMap<String, usize>) -> Value {
        match value {
            Value::Func(param, body) =>
                Value::Func(param, Box::new(substitute(*body, env, num_symbols))),
            Value::Apply(fun, arg) => {
                let fun = substitute(*fun, env, num_symbols);
                let arg = substitute(*arg, env, num_symbols);
                match fun {
                    Value::Func(param, body) if num_symbols.get(&param) == Some(&1) => {
                        env.insert(param.clone(), arg);
                        let evaluated = substitute(*body, env, num_symbols);
                        env.remove(&param);
                        evaluated
                    },
                    fun => Value::Apply(Box::new(fun), Box::new(arg)),
                }
            },
            Value::SYMBOL(s) => {
                if let Some(val) = env.remove(&s) {
                    val
                } else {
                    Value::SYMBOL(s)
                }
            },
            v => v,
        }
    }
    let substituted = {
        let mut env = HashMap::new();
        let num_symbols = count_symbols(&value);
        substitute(value, &mut env, &num_symbols)
    };
    fn relabel(value: Value, env: &mut HashMap<String, Value>) -> Value {
        match value {
            Value::Func(param, body) => {
                // unlike usual eval, this evaluates the body.
                Value::Func(param, Box::new(relabel(*body, env)))
            },
            Value::Apply(fun, arg) => {
                let fun = relabel(*fun, env);
                let arg = relabel(*arg, env);
                match (fun, arg) {
                    (Value::Func(param, body), a @ Value::SYMBOL(_)) => {
                        env.insert(param.clone(), a);
                        let relabeled = relabel(*body, env);
                        env.remove(&param);
                        relabeled
                    },
                    (Value::Func(param, body), a @ Value::INT(_)) => {
                        env.insert(param.clone(), a);
                        let relabeled = relabel(*body, env);
                        env.remove(&param);
                        relabeled
                    },
                    (f, a) => Value::Apply(Box::new(f), Box::new(a)),
                }
            },
            mut e @ Value::SYMBOL(_) => loop {
                match e {
                    Value::SYMBOL(s) => {
                        let val = env.get(&s);
                        if val.is_none() {
                            return Value::SYMBOL(s);
                        }
                        e = val.unwrap().clone()
                    },
                    _ => return e,
                };
            },
            v => v,
        }
    }
    let mut env = HashMap::new();
    eprintln!("Before relabel: {}", substituted);
    relabel(substituted, &mut env)
}
