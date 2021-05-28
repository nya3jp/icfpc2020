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
pub enum Value {
    Apply(Box<Value>, Box<Value>),

    SYMBOL(String),
    INT(isize),
    ADD,
    B,
    C,
    CAR,
    CDR,
    CONS,
    DIV,
    EQ,
    I,
    ISNIL,
    LT,
    MUL,
    NEG,
    NIL,
    S,
    T,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub value: Box<Value>,
}

impl Definition {
    pub fn new(name: String, value: Box<Value>) -> Self {
        Self{name, value}
    }
}