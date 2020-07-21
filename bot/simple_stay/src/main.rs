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
    let start_params = rust_game_base::Param { energy: 300, laser_power: 0, cool_down_per_turn: 8, life: 1 };
    start_params
}

fn play(resp: &rust_game_base::Response) -> Vec<rust_game_base::Command> {
    let mut commands = vec![];
    let current_state = resp.current_state.as_ref().unwrap();
    for (machine, _) in current_state.machines.iter() {
        match actions::stay(current_state, machine.machine_id) {
            Some(command) => commands.push(command),
            None => (),
        }
    }
    commands
}
