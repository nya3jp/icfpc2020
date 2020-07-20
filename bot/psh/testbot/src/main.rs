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

    // laser_test(resp)
    // heat_test1(resp)
    // move_test1(resp)
    survive_test(resp)
}

fn stay_bot(resp: rust_game_base::Response) -> Result<()> {
    let self_role = resp.stage_data.self_role;
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param{
        energy: resp.stage_data.initialize_param.total_cost - 8 * 12 - 2,
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
        energy: resp.stage_data.initialize_param.total_cost - 8 * 12 - 2,
        laser_power: 0,
        cool_down_per_turn: 8,
        life: 1
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(), rust_game_base::Role::ATTACKER);
    eprintln!("self machine_ids: {:?}", self_machine_ids);

    // find target machines.
    let opponent_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(), rust_game_base::Role::DEFENDER);
    eprintln!("oppo machine_ids: {:?}", opponent_machine_ids);

    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let target = rust_game_base::get_machine_by_id(
            res.current_state.as_ref().unwrap(),
            opponent_machine_ids[0]);
        eprintln!("moving_to: {:?}", target.unwrap().position);
        let next_actions = self_machine_ids.iter()
            .filter_map(
                |id| actions::move_to3(res.current_state.as_ref().unwrap(), *id, target.unwrap().position))
            .collect::<Vec<_>>();
        res = rust_game_base::send_command_request(
            &mut next_actions.into_iter())?;
    }
}

fn laser_test(resp: rust_game_base::Response) -> Result<()> {
    // Must be an attacker.
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param{
        energy: resp.stage_data.initialize_param.total_cost
            - 4 * 64
            - 16 * 12
            - 2,
        laser_power: 64,
        cool_down_per_turn: 16,
        life: 1
    })?;

    // find self machines.
    let self_machine_ids = rust_game_base::get_roled_machine_ids(
        res.current_state.as_ref().unwrap(), rust_game_base::Role::ATTACKER);
    eprintln!("self machine_ids: {:?}", self_machine_ids);

    let mut turn = 0;
    eprintln!("send_command_request");
    loop {
        if res.current_game_state == rust_game_base::CurrentGameState::END {
            return Ok(());
        }
        let mut next_actions = self_machine_ids.iter()
            .filter_map(
                |id| actions::stay(res.current_state.as_ref().unwrap(), *id))
            .collect::<Vec<_>>();
        if turn == 0 {
            next_actions.append(&mut self_machine_ids.iter()
                .filter_map(
                    |id| {
                        let machine = rust_game_base::get_machine_by_id(
                            res.current_state.as_ref().unwrap(), *id).unwrap();
                        actions::laser(
                            res.current_state.as_ref().unwrap(),
                            *id,
                            machine.position + rust_game_base::Point::new(0, 10),
                        )
                    })
                .collect::<Vec<_>>());
        }
        res = rust_game_base::send_command_request(
            &mut next_actions.into_iter())?;
    }

}

fn survive_test(resp: rust_game_base::Response) -> Result<()> {
    let self_role = resp.stage_data.self_role;
    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param{
        energy: resp.stage_data.initialize_param.total_cost - 8 * 12 - 2,
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
                |id| {
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
        res = rust_game_base::send_command_request(
            &mut next_actions.into_iter())?;
    }
}
