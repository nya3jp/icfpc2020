use std::io;

const JOIN_REQUEST_TAG: i128 = 2;
const START_REQUEST_TAG: i128 = 3;

// move command
// ( ( 0, ( 0, ( ( X, Y ), nil ) ) ), nil )
//
//   move (-1, 0)
//   ( ( 0, ( 0, ( ( -1, 0 ), nil ) ) ), nil )
//   send: ( 4, ( 2294550191781414755, ( ( ( 0, ( 0, ( ( -1, 0 ), nil ) ) ), nil ), nil ) ) ) => ( 1, ( 1, ( ( 12, ( 0, ( ( 512, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ), ( ( 1, ( nil, ( ( ( ( 1, ( 1, ( ( 33, 6 ), ( ( 0, 0 ), ( ( 78, ( 0, ( 0, ( 1, nil ) ) ) ), ( 0, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), ( ( ( 0, ( 0, ( ( 17, 0 ), ( ( 1, 0 ), ( ( 2, ( 0, ( 0, ( 1, nil ) ) ) ), ( 8, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( ( ( 0, ( ( -1, 0 ), nil ) ), nil ), nil ) ), nil ) ), nil ) ) ), nil ) ) ) )
//
//   move (-1, -1)
//   send: ( 4, ( 2294550191781414755, ( ( ( 0, ( 0, ( ( -1, -1 ), nil ) ) ), nil ), nil ) ) ) => ( 1, ( 1, ( ( 12, ( 0, ( ( 512, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ), ( ( 2, ( nil, ( ( ( ( 1, ( 1, ( ( 33, 6 ), ( ( 0, 0 ), ( ( 78, ( 0, ( 0, ( 1, nil ) ) ) ), ( 0, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), ( ( ( 0, ( 0, ( ( 19, 1 ), ( ( 2, 1 ), ( ( 1, ( 0, ( 0, ( 1, nil ) ) ) ), ( 16, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( ( ( 0, ( ( -1, -1 ), nil ) ), nil ), nil ) ), nil ) ), nil ) ) ), nil ) ) ) )
//
//   Move twice
//     send: ( 4, ( 2124087914363152034, ( ( ( 0, ( 0, ( ( -1, 0 ), nil ) ) ), nil ), nil ) ) ) => ( 1, ( 1, ( ( 8, ( 0, ( ( 512, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ), ( ( 2, ( nil, ( ( ( ( 1, ( 1, ( ( 38, 0 ), ( ( 0, 0 ), ( ( 32, ( 0, ( 0, ( 1, nil ) ) ) ), ( 0, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), ( ( ( 0, ( 0, ( ( 19, 0 ), ( ( 2, 0 ), ( ( 9, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( ( ( 0, ( ( -1, 0 ), nil ) ), nil ), nil ) ), nil ) ), nil ) ) ), nil ) ) ) )
//     send: ( 4, ( 2124087914363152034, ( ( ( 0, ( 0, ( ( -1, 0 ), nil ) ) ), nil ), nil ) ) ) => ( 1, ( 1, ( ( 8, ( 0, ( ( 512, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ), ( ( 1, ( nil, ( ( ( ( 1, ( 1, ( ( 38, 0 ), ( ( 0, 0 ), ( ( 32, ( 0, ( 0, ( 1, nil ) ) ) ), ( 0, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), ( ( ( 0, ( 0, ( ( 17, 0 ), ( ( 1, 0 ), ( ( 14, ( 0, ( 0, ( 1, nil ) ) ) ), ( 60, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( ( ( 0, ( ( -1, 0 ), nil ) ), nil ), nil ) ), nil ) ), nil ) ) ), nil ) ) ) )
//
// self destruct command
// ( ( 1, ( 0, nil ) ), nil )
//
// beam
// ( ( 2, ( 0, ( ( X, Y ), ( power=86, nil ) ) ) ), nil )
//   send: ( 4, ( 7346510104303901170, ( ( ( 2, ( 0, ( ( 48, 0 ), ( 86, nil ) ) ) ), nil ), nil ) ) ) =>
//  ( 1, ( 1, (
//              ( 4, ( 0, ( ( 512, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ),
//              ( ( 1, ( nil, ( (
//                               ( ( 1, ( 1, ( ( 48, 0 ), ( ( 0, 0 ), ( ( 196, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ),
//                                 ( ( ( 0, ( 0, ( ( 16, 0 ), ( ( 0, 0 ), ( ( 0, ( power_left=75, ( 11, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( ( ( 2, ( ( 48, 0 ), ( 86, ( 227, ( 4, nil ) ) ) ) ), nil ), nil ) ), nil ) ), nil ) ) ), nil ) ) ) )
//
//   send: ( 4, ( 7346510104303901170, ( ( ( 2, ( 0, ( ( 48, 0 ), ( 75, nil ) ) ) ), nil ), nil ) ) ) => ( 1, ( 1, ( ( 4, ( 0, ( ( 512, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ), ( ( 2, ( nil, ( ( ( ( 1, ( 1, ( ( 48, 0 ), ( ( 0, 0 ), ( ( 2, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), ( ( ( 0, ( 0, ( ( 16, 0 ), ( ( 0, 0 ), ( ( 0, ( 11, ( 11, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( ( ( 2, ( ( 48, 0 ), ( 75, ( 194, ( 4, nil ) ) ) ) ), nil ), nil ) ), nil ) ), nil ) ) ), nil ) ) ) )
//
//  multiple opponents:
//  send: ( 3, ( 8163103398110191786, ( nil, nil ) ) ) =>
//  ( 1, ( 1, (
//              ( 8, ( 1, ( ( 448, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ),
//              ( ( 0, ( nil, ( (
//                               ( ( 1, ( 0, ( ( 16 = X, 0 = Y ), ( ( 0, 0 ), ( ( 0, ( 16, ( 16, ( 1, nil ) ) ) ), ( 0, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ),
//                                 ( ( ( 0, ( 1, ( ( 44 = X, 0 = Y ), ( ( -3, 0 ), ( ( 24, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) )) ) ) ) ) ), ( nil, nil ) ),
//                                 ( ( ( 0, ( 2, ( ( 48 = X, 32 = Y ), ( ( -6, -6 ), ( ( 24, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ),
//                                 ( ( ( 0, ( 3, ( ( 16 = X, 35 = Y ), ( ( 0, -5 ), ( ( 24, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), nil ) ) ) ), nil ) ) ), nil ) ) ) )
//
//  send: ( 4, ( 8163103398110191786, ( nil, nil ) ) ) =>
//  ( 1, ( 1, (
//              ( 8, ( 1, ( ( 448, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ),
//              ( ( 1, ( nil, ( (
//                               ( ( 1, ( 0, ( ( 16, 0 ), ( ( 0, 0 ), ( ( 0, ( 16, ( 16, ( 1, nil ) ) ) ), ( 0, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ),
//                                 ( ( ( 0, ( 1, ( p=( 41, 0 ), ( v=( -3, 0 ), ( ( 24, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ),
//                                 ( ( ( 0, ( 2, ( p=( 42, 26 ), ( v=( -6, -6 ), ( ( 24, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ),
//                                 ( ( ( 0, ( 3, ( p=( 16, 30 ), ( v=( 0, -5 ), ( ( 24, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), nil ) ) ) ), nil ) ) ), nil ) ) ) )
//
//  send: ( 4, ( 4515683523718609980, ( ( ( 2, ( 0, ( ( 23, 8 ), ( 16, nil ) ) ) ), nil ), nil ) ) ) =>
//  ( 1, ( 1, ( ( 8, ( 1, ( ( 448, ( 1, ( 64, nil ) ) ), ( nil, ( nil, nil ) ) ) ) ), ( ( 4, ( nil, ( ( ( ( 1, ( 0, ( ( 16, 0 ), ( ( 0, 0 ), ( ( 0, ( 16, ( 16, ( 1, nil ) ) ) ), ( 0, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( ( ( 2, ( ( 23, 8 ), ( 16, ( 29, ( 4, nil ) ) ) ) ), nil ), nil ) ), ( ( ( 0, ( 2, ( ( 24, 8 ), ( ( -6, -6 ), ( ( 17, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), ( ( ( 0, ( 3, ( ( 16, 15 ), ( ( 0, -5 ), ( ( 24, ( 0, ( 0, ( 1, nil ) ) ) ), ( 64, ( 64, ( 1, nil ) ) ) ) ) ) ) ), ( nil, nil ) ), nil ) ) ), nil ) ) ), nil ) ) ) )

pub fn send_join_request() -> GameState {
    let player_key: i128 = std::env::args().nth(1).unwrap().parse().unwrap();
    send_and_receive_game_state(&Value::Cons(
        Box::new(Value::Int(JOIN_REQUEST_TAG)),
        Box::new(Value::Cons(
            Box::new(Value::Int(player_key)),
            Box::new(Value::Cons(Box::new(Value::Nil), Box::new(Value::Nil))),
        )),
    ))
}

pub fn send_start_request(param1: i32, param2: i32, param3: i32, param4: i32) -> GameState {
    let player_key: i128 = std::env::args().nth(1).unwrap().parse().unwrap();
    let params = Value::Cons(
        Box::new(Value::Int(param1 as i128)),
        Box::new(Value::Cons(
            Box::new(Value::Int(param2 as i128)),
            Box::new(Value::Cons(
                Box::new(Value::Int(param3 as i128)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(param4 as i128)),
                    Box::new(Value::Nil),
                )),
            )),
        )),
    );
    send_and_receive_game_state(&Value::Cons(
        Box::new(Value::Int(START_REQUEST_TAG)),
        Box::new(Value::Cons(
            Box::new(Value::Int(player_key)),
            Box::new(Value::Cons(Box::new(params), Box::new(Value::Nil))),
        )),
    ))
}

#[derive(Debug)]
pub struct GameState {
    state1: Value,
    state2: Value,
}

fn send_and_receive_game_state(val: &Value) -> GameState {
    let state = send_and_receive(val);
    GameState {
        state1: Value::Nil,
        state2: Value::Nil,
    }
}

fn send_and_receive(val: &Value) -> Value {
    println!("{}", modulate_to_string(&val));
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    demodulate_from_string(&resp).unwrap()
}

#[derive(Debug)]
enum Value {
    Int(i128),
    Nil,
    Cons(Box<Value>, Box<Value>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            &Value::Int(n) => format!("{}", n),
            Value::Nil => format!("'nil"),
            Value::Cons(x, y) => format!("({} . {})", x.to_string(), y.to_string()),
        }
    }
}

fn modulate_to_string(val: &Value) -> String {
    let mut v = vec![];
    modulate(&val, &mut v);
    v.iter().map(|x| if *x { "1" } else { "0" }).collect()
}

fn demodulate_from_string(s: &str) -> Option<Value> {
    let vb: Vec<bool> = s.chars().map(|x| x == '1').collect();
    demodulate(&mut vb.into_iter())
}

fn modulate(val: &Value, v: &mut Vec<bool>) {
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

fn demodulate(it: &mut impl Iterator<Item = bool>) -> Option<Value> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_demod() {
        for tc in [
            (
                "110110000111011111100001001111110100110000",
                "(1 . (81740 . 'nil))",
            ),
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
