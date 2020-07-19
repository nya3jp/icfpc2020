use std::io;

const JOIN_REQUEST_TAG: i128 = 2;
const START_REQUEST_TAG: i128 = 3;
const COMMAND_REQUEST_TAG: i128 = 4;

const THRUST_COMMAND: i128 = 0;
const SELF_DESTRUCT_COMMAND: i128 = 1;
const BEAM_COMMAND: i128 = 2;

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
    let is_tutorial: bool = std::env::vars().any(|(key, _)| key == "TUTORIAL_MODE");
    let params = if is_tutorial {
        Value::Nil
    } else {
        Value::Cons(
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
        )
    };
    send_and_receive_game_state(&Value::Cons(
        Box::new(Value::Int(START_REQUEST_TAG)),
        Box::new(Value::Cons(
            Box::new(Value::Int(player_key)),
            Box::new(Value::Cons(Box::new(params), Box::new(Value::Nil))),
        )),
    ))
}

pub fn send_command_request(it: &mut impl Iterator<Item = Command>) -> GameState {
    let commands = it.fold(Value::Nil, |acc, x| {
        Value::Cons(Box::new(x.to_value()), Box::new(acc))
    });
    let player_key: i128 = std::env::args().nth(1).unwrap().parse().unwrap();
    send_and_receive_game_state(&Value::Cons(
        Box::new(Value::Int(COMMAND_REQUEST_TAG)),
        Box::new(Value::Cons(
            Box::new(Value::Int(player_key)),
            Box::new(Value::Cons(Box::new(commands), Box::new(Value::Nil))),
        )),
    ))
}

#[derive(Debug)]
pub enum Command {
    // Thrust(ShipNum, X, Y)
    Thrust(i8, i8, i8),
    // SelfDestruct(ShipNum)
    SelfDestruct(i8),
    // Beam(ShipNum, X, Y, Power)
    Beam(i8, i16, i16, i8),
}

impl Command {
    fn to_value(&self) -> Value {
        match self {
            //   send: [0, SHIP_NUM, (X . Y)]
            &Command::Thrust(ship_num, x, y) => Value::Cons(
                Box::new(Value::Int(THRUST_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Cons(
                        Box::new(Value::Cons(
                            Box::new(Value::Int(x as i128)),
                            Box::new(Value::Int(y as i128)),
                        )),
                        Box::new(Value::Nil),
                    )),
                )),
            ),
            // send [1, SHIP_NUM]
            &Command::SelfDestruct(ship_num) => Value::Cons(
                Box::new(Value::Int(SELF_DESTRUCT_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Nil),
                )),
            ),
            // send: [2, SHIP_NUM, ( X . Y ), POWER] =>
            &Command::Beam(ship_num, x, y, power) => Value::Cons(
                Box::new(Value::Int(BEAM_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Cons(
                        Box::new(Value::Cons(
                            Box::new(Value::Int(x as i128)),
                            Box::new(Value::Int(y as i128)),
                        )),
                        Box::new(Value::Cons(
                            Box::new(Value::Int(power as i128)),
                            Box::new(Value::Nil),
                        )),
                    )),
                )),
            ),
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    state1: Value,
    state2: Value,
}

fn send_and_receive_game_state(val: &Value) -> GameState {
    send_and_receive(val);
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
