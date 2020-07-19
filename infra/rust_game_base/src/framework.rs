use self::super::game::*;
use self::super::value::*;
use io::Write;
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
    match *val {
        Value::Int(0) => CurrentGameState::START,
        Value::Int(1) => CurrentGameState::PLAYING,
        Value::Int(2) => CurrentGameState::END,
        _ => panic!(),
    }
}

fn parse_obstacle(val: Value) -> Option<Obstacle> {
    to_option(val).map(|val| match to_vec(val.clone()).as_slice() {
        [gravity_radius, stage_half_size] => Obstacle {
            gravity_radius: to_int(gravity_radius) as usize,
            stage_half_size: to_int(stage_half_size) as usize,
        },
        _ => panic!("unexpected value: ".to_string() + &val.to_string()),
    })
}

fn parse_stage_data(val: Value) -> StageData {
    match to_vec(val.clone()).as_slice() {
        [total_turns, role, _2, obstacle, _3] => match to_vec(_2.clone()).as_slice() {
            [_20, _21, _22] => StageData {
                total_turns: to_int(total_turns) as usize,
                role: to_int(role) as isize,
                _2: (
                    to_int(_20) as isize,
                    to_int(_21) as isize,
                    to_int(_22) as isize,
                ),
                obstacle: parse_obstacle(obstacle.clone()),
                _3: to_vec(_3.clone())
                    .into_iter()
                    .map(|val| to_int(&val) as isize)
                    .collect(),
            },
            _ => panic!("unexpected value: ".to_string() + &_2.to_string()),
        },
        _ => panic!("unexpected value: ".to_string() + &val.to_string()),
    }
}

fn parse_position(val: Value) -> (isize, isize) {
    match val {
        Value::Cons(x, y) => (to_int(&*x) as isize, to_int(&*y) as isize),
        _ => panic!("Unexpected value: ".to_string() + &val.to_string()),
    }
}

fn parse_params(val: Value) -> Param {
    match to_vec(val.clone()).as_slice() {
        [energy, laser_power, cool_down_per_turn, life] => Param {
            energy: to_int(energy) as usize,
            laser_power: to_int(laser_power) as usize,
            cool_down_per_turn: to_int(cool_down_per_turn) as usize,
            life: to_int(life) as usize,
        },
        _ => panic!("unexpected value: ".to_string() + &val.to_string()),
    }
}

fn parse_machine(val: Value) -> Machine {
    match to_vec(val.clone()).as_slice() {
        [team_id, machine_id, position, velocity, params, heat, _1, _2] => Machine {
            team_id: to_int(team_id) as isize,
            machine_id: to_int(machine_id) as isize,
            position: parse_position(position.clone()),
            velocity: parse_position(velocity.clone()),
            params: parse_params(params.clone()),
            heat: to_int(heat) as usize,
            _1: to_int(_1) as isize,
            _2: to_int(_2) as isize,
        },
        _ => panic!("unexpected value: ".to_string() + &val.to_string()),
    }
}

fn parse_action_result(val: Value) -> Option<ActionResult> {
    let vals = to_vec(val.clone());
    if vals.is_empty() {
        return None;
    } else {
        let action_result = match (to_int(&vals[0].clone()), vals.as_slice()) {
            (0, [_, a]) => ActionResult::Thruster {
                a: parse_position(a.clone()),
            },
            (1, [_, power, area]) => ActionResult::Bomb {
                power: to_int(power) as usize,
                area: to_int(area) as usize,
            },
            (2, [_, opponent]) => ActionResult::Laser {
                opponent: parse_position(opponent.clone()),
            },
            (3, [_, params]) => ActionResult::Split {
                params: parse_params(params.clone()),
            },
            _ => panic!("unexpected value: ".to_string() + &val.to_string()),
        };
        Some(action_result)
    }
}

fn parse_machine_and_action_result(val: Value) -> (Machine, Option<ActionResult>) {
    match to_vec(val.clone()).as_slice() {
        [machine, action_result] => (
            parse_machine(machine.clone()),
            parse_action_result(action_result.clone()),
        ),
        _ => panic!("unexpected value: ".to_string() + &val.to_string()),
    }
}

fn parse_current_state(val: Value) -> Option<CurrentState> {
    match to_vec(val.clone()).as_slice() {
        [turn, obstacle, machines] => Some(CurrentState {
            turn: to_int(turn) as usize,
            obstacle: parse_obstacle(obstacle.clone()),
            machines: to_vec(machines.clone())
                .into_iter()
                .map(|val| parse_machine_and_action_result(val))
                .collect(),
        }),
        [] => None,
        _ => panic!("unexpected value: ".to_string() + &val.to_string()),
    }
}

fn parse_response(val: Value) -> Response {
    match to_vec(val.clone()).as_slice() {
        [one, current_game_state, stage_data, current_state] => Response {
            _1: to_int(&one) as usize,
            current_game_state: parse_current_game_state(current_game_state),
            stage_data: parse_stage_data(stage_data.clone()),
            current_state: parse_current_state(current_state.clone()),
        },
        _ => panic!("unexpected value: ".to_string() + &val.to_string()),
    }
}

fn send_and_receive_game_state(val: &Value) -> Response {
    println!("{}", modulate_to_string(&val));
    io::stdout().flush();
    eprintln!("send: {}", val.to_string());
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    let resp = demodulate_from_string(&resp).unwrap();
    let resp = parse_response(resp);
    eprintln!("recieve: {:?}", resp);
    resp
}
