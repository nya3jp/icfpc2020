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

    eprintln!("send_start_request");
    let mut res = rust_game_base::send_start_request(&rust_game_base::Param{
        energy: 350,
        laser_power: 0,
        cool_down_per_turn: 8,
        life: 1
    })?;

    // find self machines.
    let self_machine_ids = res.current_state.as_ref().unwrap().machines.iter()
        .filter_map(|(m, _)| if m.role == self_role {
            Some(m.machine_id)
        } else {
            None
        })
        .collect::<Vec<_>>();
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
