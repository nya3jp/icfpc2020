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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    SYMBOL(String),
    INT(isize),
    ADD,
    AP,
    B,
    C,
    CAR,
    CDR,
    CONS,
    DIV,
    EQ,
    EQUAL,
    I,
    ISNIL,
    LT,
    MUL,
    NEG,
    NIL,
    S,
    T,
}

fn to_token(s: &str) -> Token {
    match s {
        "add" => Token::ADD,
        "ap" => Token::AP,
        "b" => Token::B,
        "c" => Token::C,
        "car" => Token::CAR,
        "cdr" => Token::CDR,
        "cons" => Token::CONS,
        "div" => Token::DIV,
        "eq" => Token::EQ,
        "i" => Token::I,
        "isnil" => Token::ISNIL,
        "lt" => Token::LT,
        "mul" => Token::MUL,
        "neg" => Token::NEG,
        "nil" => Token::NIL,
        "s" => Token::S,
        "t" => Token::T,
        "=" => Token::EQUAL,
        _ => match s.parse::<isize>() {
            Ok(num) => Token::INT(num),
            Err(_) => Token::SYMBOL(s.to_string()),
        }
    }
}

pub fn lex(s: &str) -> Vec<Token> {
    s.split_whitespace().map(to_token).collect()
}
