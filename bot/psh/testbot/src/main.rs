#![allow(non_snake_case, unused, non_upper_case_globals)]
extern crate rust_game_base;

use anyhow::Result;
use std::env;
use rust_game_base::actions;

fn main() -> Result<()> {
    eprintln!("send_join_request");
    let resp = rust_game_base::send_join_request()?;

    let self_role = resp.stage_data.self_role;
    eprintln!("Self Role: {:?}", self_role);

    if self_role == rust_game_base::Role::DEFENDER {
        return stay_bot(resp);
    }

    // heat_test1(resp)
    move_test1(resp)
}

fn stay_bot(resp: rust_game_base::Response) -> Result<()> {
    let self_role = resp.stage_data.self_role;
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param{
        energy: 350,  // TODO.
        laser_power: 0,
        cool_down_per_turn: 8,
        life: 1
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(), self_role);
    eprintln!("machine_ids: {:?}", self_machine_ids);

    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let next_actions = self_machine_ids.iter()
            .filter_map(
                |id| actions::stay(res.current_state.as_ref().unwrap(), *id))
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(
            &mut next_actions.into_iter())?;
    }
}

fn heat_test1(resp: rust_game_base::Response) -> Result<()> {
    // Must be an attacker.
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param{
        energy: 10,  // 64 / 8 + 1.
        laser_power: 0,
        cool_down_per_turn: 1,  // must be zero.
        life: 8
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(), rust_game_base::Role::ATTACKER);
    eprintln!("machine_ids: {:?}", self_machine_ids);

    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let next_actions = self_machine_ids.iter()
            .filter_map(
                |id| actions::stay(res.current_state.as_ref().unwrap(), *id))
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(
            &mut next_actions.into_iter())?;
    }
}

fn move_test1(resp: rust_game_base::Response) -> Result<()> {
    // Must be an attacker.
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param{
        energy: 510,
        laser_power: 0,
        cool_down_per_turn: 0,
        life: 1
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(), rust_game_base::Role::ATTACKER);
    eprintln!("machine_ids: {:?}", self_machine_ids);

    let target_pos = rust_game_base::get_machine_by_id(
        res.current_state.as_ref().unwrap(),
        self_machine_ids[0]).unwrap().position + (rust_game_base::Point{x: 10, y: 10});

    eprintln!("moving_to: {:?}", target_pos);
    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let next_actions = self_machine_ids.iter()
            .filter_map(
                |id| actions::move_to(res.current_state.as_ref().unwrap(), *id, target_pos))
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(
            &mut next_actions.into_iter())?;
    }
}
