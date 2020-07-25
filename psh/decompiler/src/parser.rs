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

use super::ast;
use super::lexer;

fn parse_value(iter: &mut dyn Iterator<Item = &lexer::Token>) -> ast::Value {
    match iter.next() {
        Some(lexer::Token::AP) => {
            let fun = parse_value(iter);
            let arg = parse_value(iter);
            ast::Value::Apply(Box::new(fun), Box::new(arg))
        },
        Some(lexer::Token::SYMBOL(s)) => ast::Value::SYMBOL(s.clone()),
        Some(lexer::Token::INT(v)) => ast::Value::INT(*v),
        Some(lexer::Token::ADD) => ast::Value::ADD,
        Some(lexer::Token::B) => ast::Value::B,
        Some(lexer::Token::C) => ast::Value::C,
        Some(lexer::Token::CAR) => ast::Value::CAR,
        Some(lexer::Token::CDR) => ast::Value::CDR,
        Some(lexer::Token::CONS) => ast::Value::CONS,
        Some(lexer::Token::DIV) => ast::Value::DIV,
        Some(lexer::Token::EQ) => ast::Value::EQ,
        Some(lexer::Token::I) => ast::Value::I,
        Some(lexer::Token::ISNIL) => ast::Value::ISNIL,
        Some(lexer::Token::LT) => ast::Value::LT,
        Some(lexer::Token::MUL) => ast::Value::MUL,
        Some(lexer::Token::NEG) => ast::Value::NEG,
        Some(lexer::Token::NIL) => ast::Value::NIL,
        Some(lexer::Token::S) => ast::Value::S,
        Some(lexer::Token::T) => ast::Value::T,
        None => panic!("Unexpected EOL"),
        x => panic!("Unexpected token: {:?}", x),
    }
}

pub fn parse_line(line: &str) -> ast::Definition {
    let tokens = lexer::lex(line);
    let mut iter = tokens.iter();
    let name = {
        if let Some(lexer::Token::SYMBOL(name)) = iter.next() {
            name
        } else {
            panic!("Unexpected synbol: {}", line)
        }
    };
    if iter.next() != Some(&lexer::Token::EQUAL) {
        panic!("Unexpected syntax: {}", line)
    }
    ast::Definition::new(name.clone(), Box::new(parse_value(&mut iter)))
}