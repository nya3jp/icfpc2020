use self::super::game::*;
use self::super::value::*;
use std::io;

const JOIN_REQUEST_TAG: i128 = 2;
const START_REQUEST_TAG: i128 = 3;
const COMMAND_REQUEST_TAG: i128 = 4;

pub fn send_join_request() -> Response {
    let player_key: i128 = std::env::args().nth(1).unwrap().parse().unwrap();
    send_and_receive_game_state(&Value::Cons(
        Box::new(Value::Int(JOIN_REQUEST_TAG)),
        Box::new(Value::Cons(
            Box::new(Value::Int(player_key)),
            Box::new(Value::Cons(Box::new(Value::Nil), Box::new(Value::Nil))),
        )),
    ))
}

pub fn send_start_request(param1: i32, param2: i32, param3: i32, param4: i32) -> Response {
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

pub fn send_command_request(it: &mut impl Iterator<Item = Command>) -> Response {
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

fn parse_current_game_state(val: &Value) -> CurrentGameState {
    match val {
        Value::Int(0) => CurrentGameState::START,
        Value::Int(1) => CurrentGameState::PLAYING,
        Value::Int(2) => CurrentGameState::END,
        _ => panic!(),
    }
}

fn parse_stage_data(val: &Value) -> StageData {
    StageData {
        total_turns: 0,
        _1: 0,
        _2: (0, 0, 0),
        obstacle: None,
        _3: vec![],
    }
}

fn parse_current_state(val: &Value) -> CurrentState {
    CurrentState {
        turn: 0,
        obstacle: None,
        machines: vec![],
    }
}

fn parse_response(val: &Value) -> Response {
    Response {
        _1: 1,
        current_game_state: parse_current_game_state(val),
        stage_data: parse_stage_data(val),
        current_state: parse_current_state(val),
    }
}

fn send_and_receive_game_state(val: &Value) -> Response {
    parse_response(&send_and_receive(val))
}

fn send_and_receive(val: &Value) -> Value {
    println!("{}", modulate_to_string(&val));
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    demodulate_from_string(&resp).unwrap()
}
