use crate::value::Value;
use std::convert::TryInto;

pub fn int(n: impl TryInto<i128>) -> Value {
    Value::Int(n.try_into().ok().unwrap())
}

pub fn cons(car: Value, cdr: Value) -> Value {
    Value::Cons(Box::new(car), Box::new(cdr))
}

pub fn nil() -> Value {
    Value::Nil
}

macro_rules! list {
    () => { nil() };
    ($x:expr) => { cons($x, nil()) };
    ($x:expr, $($xs:expr),*) => { cons($x, list!($($xs),*)) }
}
