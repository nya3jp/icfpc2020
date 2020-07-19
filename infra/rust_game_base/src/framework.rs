use crate::game::*;
use crate::value::*;
use anyhow::{bail, Context, Result};
use std::io::{self, Write};

const JOIN_REQUEST_TAG: i128 = 2;
const START_REQUEST_TAG: i128 = 3;
const COMMAND_REQUEST_TAG: i128 = 4;

fn get_player_key() -> i128 {
    std::env::args().nth(1).unwrap().parse().unwrap()
}

pub fn send_join_request() -> Result<Response> {
    use crate::dsl::*;
    let player_key = get_player_key();
    eprintln!("send: JOIN player_key={}", player_key);
    send_and_receive_game_state(&list!(int(JOIN_REQUEST_TAG), int(player_key), nil()))
}

pub fn send_start_request(params: &Param) -> Result<Response> {
    use crate::dsl::*;
    let player_key = get_player_key();
    let is_tutorial: bool = std::env::vars().any(|(key, _)| key == "TUTORIAL_MODE");

    if is_tutorial {
        eprintln!("send: START player_key={} tutorial", player_key);
    } else {
        eprintln!("send: START player_key={} {:?}", player_key, params);
    }

    let params = if is_tutorial {
        list!()
    } else {
        list!(
            int(params.energy),
            int(params.laser_power),
            int(params.cool_down_per_turn),
            int(params.life)
        )
    };
    send_and_receive_game_state(&list!(int(START_REQUEST_TAG), int(player_key), params))
}

pub fn send_command_request(it: &mut impl Iterator<Item = Command>) -> Result<Response> {
    use crate::dsl::*;
    let player_key = get_player_key();
    let commands = it.collect::<Vec<Command>>();
    eprintln!("send: COMMAND player_key={} {:?}", player_key, commands);
    let commands = commands
        .iter()
        .fold(nil(), |acc, x| cons(x.to_value(), acc));
    send_and_receive_game_state(&list!(int(COMMAND_REQUEST_TAG), int(player_key), commands))
}

fn parse_current_game_state(val: &Value) -> CurrentGameState {
    match *val {
        Value::Int(0) => CurrentGameState::START,
        Value::Int(1) => CurrentGameState::PLAYING,
        Value::Int(2) => CurrentGameState::END,
        _ => panic!(),
    }
}

fn parse_obstacle(val: Value) -> Result<Option<Obstacle>> {
    Ok(if let Some(val) = to_option(val) {
        Some(match to_vec(val.clone())?.as_slice() {
            [gravity_radius, stage_half_size] => Obstacle {
                gravity_radius: to_int(gravity_radius)? as usize,
                stage_half_size: to_int(stage_half_size)? as usize,
            },
            _ => bail!("unexpected value: {}", val.to_string()),
        })
    } else {
        None
    })
}

fn parse_initialize_param(val: Value) -> Result<InitializeParam> {
    Ok(match to_vec(val.clone())?.as_slice() {
        [total_cost, _2, _3] => InitializeParam {
            total_cost: to_int(total_cost)? as usize,
            _2: to_int(_2)? as isize,
            _3: to_int(_3)? as isize,
        },
        _ => bail!("unexpected initialize param: {}", val.to_string()),
    })
}

fn parse_stage_data(val: Value) -> Result<StageData> {
    Ok(match to_vec(val.clone())?.as_slice() {
        [total_turns, role, initialize_param, obstacle, defender] => StageData {
            total_turns: to_int(total_turns)? as usize,
            self_role: parse_role(role.clone())?,
            initialize_param: parse_initialize_param(initialize_param.clone())?,
            obstacle: parse_obstacle(obstacle.clone())?,
            defender: match defender {
                Value::Nil => None,
                _ => Some(parse_params(defender.clone())?),
            },
        },
        _ => bail!("unexpected value: {}", val.to_string()),
    })
}

fn parse_position(val: Value) -> Result<(isize, isize)> {
    Ok(match val {
        Value::Cons(x, y) => (to_int(&*x)? as isize, to_int(&*y)? as isize),
        _ => bail!("Unexpected value: ".to_string() + &val.to_string()),
    })
}

fn parse_point(val: Value) -> Result<Point> {
    Ok(match val {
        Value::Cons(x, y) => Point {
            x: to_int(&*x)? as isize,
            y: to_int(&*y)? as isize,
        },
        _ => bail!("Unexpected value: ".to_string() + &val.to_string()),
    })
}

fn parse_role(val: Value) -> Result<Role> {
    Ok(match to_int(&val)? {
        0 => Role::ATTACKER,
        1 => Role::DEFENDER,
        _ => bail!("unexpected value: {}", val.to_string()),
    })
}

fn parse_params(val: Value) -> Result<Param> {
    Ok(match to_vec(val.clone())?.as_slice() {
        [energy, laser_power, cool_down_per_turn, life] => Param {
            energy: to_int(energy)? as usize,
            laser_power: to_int(laser_power)? as usize,
            cool_down_per_turn: to_int(cool_down_per_turn)? as usize,
            life: to_int(life)? as usize,
        },
        _ => bail!("unexpected value: {}", val.to_string()),
    })
}

fn parse_machine(val: Value) -> Result<Machine> {
    Ok(match to_vec(val.clone())?.as_slice() {
        [role, machine_id, position, velocity, params, heat, _1, _2] => Machine {
            role: parse_role(role.clone())?,
            machine_id: to_int(machine_id)? as isize,
            position: parse_point(position.clone())?,
            velocity: parse_point(velocity.clone())?,
            params: parse_params(params.clone())?,
            heat: to_int(heat)? as usize,
            _1: to_int(_1)? as isize,
            _2: to_int(_2)? as isize,
            generated_heat: 0,
            attack_heat: 0,
        },
        _ => bail!("unexpected value: ".to_string() + &val.to_string()),
    })
}

fn parse_action_result(val: Value) -> Result<Vec<ActionResult>> {
    let vals = to_vec(val.clone())?;
    vals.into_iter()
        .map(|val| -> Result<ActionResult> {
            let vals = to_vec(val)?;
            Ok(match (to_int(&vals[0])?, &vals[1..]) {
                (0, [a]) => ActionResult::Thruster {
                    a: parse_point(a.clone())?,
                },
                (1, [power, area]) => ActionResult::Bomb {
                    power: to_int(power)? as usize,
                    area: to_int(area)? as usize,
                },
                (2, [opponent, _, _, _]) => ActionResult::Laser {
                    opponent: parse_point(opponent.clone())?,
                },
                (3, [params]) => ActionResult::Split {
                    params: parse_params(params.clone())?,
                },
                _ => bail!(
                    "invalid action result: {:?}",
                    vals.into_iter().map(|r| r.to_string()).collect::<Vec<_>>()
                ),
            })
        })
        .collect::<Result<Vec<_>>>()
        .with_context(|| format!("invalid action results: {:?}", val.to_string()))
}

fn parse_machine_and_action_result(val: Value) -> Result<(Machine, Vec<ActionResult>)> {
    match to_vec(val.clone())?.as_slice() {
        [machine, action_result] => Ok((
            parse_machine(machine.clone())?,
            parse_action_result(action_result.clone())?,
        )),
        _ => bail!("unexpected value: {}", val.to_string()),
    }
}

fn parse_current_state(val: Value) -> Result<Option<CurrentState>> {
    match to_vec(val.clone())?.as_slice() {
        [turn, obstacle, machines] => Ok(Some(CurrentState {
            turn: to_int(turn)? as usize,
            obstacle: parse_obstacle(obstacle.clone())?,
            machines: to_vec(machines.clone())?
                .into_iter()
                .map(|val| parse_machine_and_action_result(val))
                .collect::<Result<Vec<_>>>()?,
        })),
        [] => Ok(None),
        _ => bail!("unexpected value: {}", val.to_string()),
    }
}

fn parse_response(val: Value) -> Result<Response> {
    match to_vec(val.clone())?.as_slice() {
        [tag, current_game_state, stage_data, current_state] => {
            if to_int(tag)? != 1 {
                bail!("tag is not 1: {}", tag.to_string());
            }
            Ok(Response {
                current_game_state: parse_current_game_state(current_game_state),
                stage_data: parse_stage_data(stage_data.clone())?,
                current_state: parse_current_state(current_state.clone())?,
            })
        }
        [tag] if to_int(tag)? == 0 => bail!("wrong request"),
        _ => bail!("unexpected response value: {}", val.to_string()),
    }
}

fn send_and_receive_game_state(val: &Value) -> Result<Response> {
    eprintln!("{:?}", val);
    println!("{}", modulate_to_string(&val));
    io::stdout().flush();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp)?;
    let resp = demodulate_from_string(&resp).unwrap();
    let resp = parse_response(resp)?;
    eprintln!("recieve: {:#?}", resp);
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let resp = demodulate_from_string("110110000111011000011111011110000100000000110101111011110001000000000110110000111011100100000000111101110000100001101110100000000011110111001100100110110101011011010101101100001000011110110000111110111000010000110111010000000001111111101100001110101111101100010111110101111111101100001010111101110011001001101101010110110101011011000010011010110111001000000110110000100111111011000101111011100011000001101111110110101011010110110010000000011111101011011000011111011100010111101101111111110100001010111101111000011001000011010110110001011011000010011010110111001000000110110000100110000000000").unwrap();
        parse_response(resp).unwrap();
    }
}
