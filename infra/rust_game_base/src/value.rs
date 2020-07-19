use anyhow::{bail, Result};

#[derive(Clone, Debug)]
pub enum Value {
    Int(i128),
    Nil,
    Cons(Box<Value>, Box<Value>),
}

fn is_list(val: &Value) -> bool {
    match val {
        Value::Int(_) => false,
        Value::Nil => true,
        Value::Cons(_, cdr) => is_list(cdr),
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            &Value::Int(n) => format!("{}", n),
            Value::Nil => format!("nil"),
            Value::Cons(x, y) => {
                if is_list(self) {
                    "(".to_string()
                        + &to_vec(self.clone())
                            .unwrap()
                            .into_iter()
                            .map(|val| val.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                        + ")"
                } else {
                    format!("({} . {})", x.to_string(), y.to_string())
                }
            }
        }
    }
}

pub fn modulate_to_string(val: &Value) -> String {
    let mut v = vec![];
    modulate(&val, &mut v);
    v.iter().map(|x| if *x { "1" } else { "0" }).collect()
}

pub fn demodulate_from_string(s: &str) -> Option<Value> {
    let vb: Vec<bool> = s.chars().map(|x| x == '1').collect();
    demodulate(&mut vb.into_iter())
}

pub fn modulate(val: &Value, v: &mut Vec<bool>) {
    match val {
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
            modulate(hd, v);
            modulate(tl, v);
        }
    }
}

pub fn demodulate(it: &mut impl Iterator<Item = bool>) -> Option<Value> {
    let t0 = it.next()?;
    let t1 = it.next()?;

    Some(match (t0, t1) {
        (false, false) => Value::Nil,
        (true, true) => {
            let x = demodulate(it)?;
            let y = demodulate(it)?;
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

// TODO
// pub fn from_vec(vals: Vec<Value>) -> Value {
// }

pub fn to_vec(val: Value) -> Result<Vec<Value>> {
    let mut val = val;
    let mut vals = Vec::new();
    loop {
        match val {
            Value::Cons(car, cdr) => {
                vals.push(*car);
                val = *cdr;
            }
            Value::Nil => break,
            _ => bail!("unexpected value: {}", val.to_string()),
        }
    }
    Ok(vals)
}

pub fn to_option(val: Value) -> Option<Value> {
    match val {
        Value::Nil => None,
        val => Some(val),
    }
}

pub fn to_int(val: &Value) -> Result<i128> {
    match val {
        Value::Int(n) => Ok(*n),
        _ => bail!("not a integer: {}", val.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_demod() {
        for tc in [
            ("110110000111011111100001001111110100110000", "(1 81740)"),
            ("010", "0"),
            ("01100001", "1"),
            ("10100001", "-1"),
        ]
        .iter()
        {
            let bin = tc.0;
            let lst = tc.1;

            assert_eq!(
                lst,
                demodulate_from_string(bin).unwrap().to_string().as_str()
            );
        }
    }

    #[test]
    fn test_mod() {
        for tc in [
            (
                "110110000111011111100001001111110100110000",
                Value::Cons(
                    Box::new(Value::Int(1)),
                    Box::new(Value::Cons(
                        Box::new(Value::Int(81740)),
                        Box::new(Value::Nil),
                    )),
                ),
            ),
            ("010", Value::Int(0)),
            ("01100001", Value::Int(1)),
            ("10100001", Value::Int(-1)),
        ]
        .iter()
        {
            let bin = tc.0;
            let lst = &tc.1;

            let v = modulate_to_string(&lst);
            assert_eq!(&v, bin);
        }
    }
}
