#![allow(non_snake_case, unused, non_upper_case_globals)]
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

extern crate rust_game_base;

use anyhow::Result;
use rust_game_base::actions;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    eprintln!("send_join_request");
    let resp = rust_game_base::send_join_request()?;

    let self_role = resp.stage_data.self_role;
    eprintln!("Self Role: {:?}", self_role);

    if self_role == rust_game_base::Role::DEFENDER {
        return stay_bot(resp);
    }

    laser_test(resp)
    // heat_test1(resp)
    // move_test1(resp)
    // survive_test(resp)
}

fn stay_bot(resp: rust_game_base::Response) -> Result<()> {
    let self_role = resp.stage_data.self_role;
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param {
        energy: resp.stage_data.initialize_param.total_cost - 8 * 12 - 2,
        laser_power: 0,
        cool_down_per_turn: 8,
        life: 1,
    })?;

    // find self machines.
    let self_machine_ids =
        rust_game_base::get_roled_machine_ids(res.current_state.as_ref().unwrap(), self_role);
    eprintln!("machine_ids: {:?}", self_machine_ids);

    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let next_actions = self_machine_ids
            .iter()
            .filter_map(|id| actions::stay(res.current_state.as_ref().unwrap(), *id))
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(&mut next_actions.into_iter())?;
    }
}

fn heat_test1(resp: rust_game_base::Response) -> Result<()> {
    // Must be an attacker.
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param {
        energy: 10, // 64 / 8 + 1.
        laser_power: 0,
        cool_down_per_turn: 1, // must be zero.
        life: 8,
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(),
        rust_game_base::Role::ATTACKER,
    );
    eprintln!("machine_ids: {:?}", self_machine_ids);

    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let next_actions = self_machine_ids
            .iter()
            .filter_map(|id| actions::stay(res.current_state.as_ref().unwrap(), *id))
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(&mut next_actions.into_iter())?;
    }
}

fn move_test1(resp: rust_game_base::Response) -> Result<()> {
    // Must be an attacker.
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param {
        energy: resp.stage_data.initialize_param.total_cost - 8 * 12 - 2,
        laser_power: 0,
        cool_down_per_turn: 8,
        life: 1,
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(),
        rust_game_base::Role::ATTACKER,
    );
    eprintln!("self machine_ids: {:?}", self_machine_ids);

    // find target machines.
    let opponent_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(),
        rust_game_base::Role::DEFENDER,
    );
    eprintln!("oppo machine_ids: {:?}", opponent_machine_ids);

    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let target = rust_game_base::get_machine_by_id(
            res.current_state.as_ref().unwrap(),
            opponent_machine_ids[0],
        );
        eprintln!("moving_to: {:?}", target.unwrap().position);
        let next_actions = self_machine_ids
            .iter()
            .filter_map(|id| {
                actions::move_to3(
                    res.current_state.as_ref().unwrap(),
                    *id,
                    target.unwrap().position,
                )
            })
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(&mut next_actions.into_iter())?;
    }
}

pub fn laser_free(
    state: &rust_game_base::CurrentState,
    machine_id: isize,
    target: rust_game_base::Point,
    intensity: isize,
) -> Option<rust_game_base::Command> {
    let machine = rust_game_base::get_machine_by_id(state, machine_id).unwrap();
    return Some(rust_game_base::Command::Beam(machine_id, target, intensity));
}

fn laser_test(resp: rust_game_base::Response) -> Result<()> {
    let mut logfile = File::create("laser.log")?;
    let mut dx = 0;
    let mut dy = 20;
    let mut pow = 64;
    for (key, value) in std::env::vars() {
        if key == "DX" {
            dx = value.parse().unwrap()
        }
        if key == "DY" {
            dy = value.parse().unwrap()
        }
        if key == "POW" {
            pow = value.parse().unwrap()
        }
    }

    // Must be an attacker.
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param {
        energy: resp.stage_data.initialize_param.total_cost - 4 * 64 - 16 * 12 - 2,
        laser_power: 64,
        cool_down_per_turn: 16,
        life: 1,
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(),
        rust_game_base::Role::ATTACKER,
    );
    eprintln!("self machine_ids: {:?}", self_machine_ids);

    let mut turn = 0;
    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let cstate = res.current_state.unwrap();
        for mpair in &cstate.machines {
            let actionresults = &mpair.1;
            for a in actionresults {
                match a {
                    rust_game_base::ActionResult::Laser {
                        opponent,
                        power,
                        intensity,
                        _3,
                    } => write!(&mut logfile, "pow={} int={} _3={}\n", power, intensity, _3)?,
                    _ => (),
                }
            }
        }
        let mut next_actions = self_machine_ids
            .iter()
            .filter_map(|id| actions::stay(&cstate, *id))
            .collect::<Vec<_>>();
        if turn % 5 == 0 {
            let relvec = rust_game_base::Point::new(dx + turn / 5, dy);
            next_actions.append(
                &mut self_machine_ids
                    .iter()
                    .filter_map(|id| {
                        let machine = rust_game_base::get_machine_by_id(&cstate, *id).unwrap();
                        laser_free(&cstate, *id, machine.position + relvec, pow - turn / 5 * 4)
                    })
                    .collect::<Vec<_>>(),
            );
            write!(&mut logfile, "x={} y={} ", relvec.x, relvec.y)?
        }
        turn += 1;
        res = rust_game_base::send_command_request(&mut next_actions.into_iter())?;
    }
}

fn survive_test(resp: rust_game_base::Response) -> Result<()> {
    let self_role = resp.stage_data.self_role;
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param {
        energy: resp.stage_data.initialize_param.total_cost - 8 * 12 - 2,
        laser_power: 0,
        cool_down_per_turn: 8,
        life: 1,
    })?;

    // find self machines.
    let self_machine_ids =
        rust_game_base::get_roled_machine_ids(res.current_state.as_ref().unwrap(), self_role);
    eprintln!("machine_ids: {:?}", self_machine_ids);

    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let next_actions = self_machine_ids
            .iter()
            .filter_map(|id| {
                let p = actions::make_surviving_path(
                    &res.stage_data,
                    res.current_state.as_ref().unwrap(),
                    *id,
                    7,
                );
                if let Some(path) = p {
                    path[0]
                } else {
                    actions::stay(res.current_state.as_ref().unwrap(), *id)
                }
            })
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(&mut next_actions.into_iter())?;
    }
}
