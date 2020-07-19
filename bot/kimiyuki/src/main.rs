#![allow(non_snake_case, unused, non_upper_case_globals)]

extern crate rust_game_base;
#[macro_use]
extern crate itertools;

use rust_game_base::Point;
use rust_game_base::actions;

fn main() {
    let start_params = initialize();
    let resp = rust_game_base::send_join_request();
    let mut resp = rust_game_base::send_start_request(&start_params).unwrap();
    while resp.current_game_state != rust_game_base::CurrentGameState::END {
        let commands = play(&resp);
        resp = rust_game_base::send_command_request(&mut commands.into_iter()).unwrap();
    }
}

fn initialize() -> rust_game_base::Param {
    let start_params = rust_game_base::Param { energy: 400, laser_power: 0, cool_down_per_turn: 2, life: 1 };
    start_params
}

fn play(resp: &rust_game_base::Response) -> Vec<rust_game_base::Command> {
    let mut commands = vec![];
    let current_state = resp.current_state.as_ref().unwrap();
    if current_state.turn % 4 != 0 {
        for (machine, _) in current_state.machines.iter() {
            match actions::stay(current_state, machine.machine_id) {
                Some(command) => commands.push(command),
                None => (),
            }
        }
    }
    commands
}
