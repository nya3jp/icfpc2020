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
    let start_params = rust_game_base::Param { energy: 300, laser_power: 0, cool_down_per_turn: 10, life: 1 };
    start_params
}

fn play(resp: &rust_game_base::Response) -> Vec<rust_game_base::Command> {
    let mut commands = vec![];
    let current_state = resp.current_state.as_ref().unwrap();
    let mut self_machines = vec![];
    let mut opponent_machines = vec![];
    for (machine, _) in current_state.machines.iter() {
        if machine.role == resp.stage_data.self_role {
            self_machines.push(machine);
        } else {
            opponent_machines.push(machine);
        }
    }
    for machine in self_machines.iter() {
        let gravity = actions::get_gravity(current_state, machine.machine_id);
        let mut a = Point { x: 0, y: 0 };
        if gravity.x == -1 {
            a.x -= 1;
            a.y += 1;
        } else if gravity.y == 1 {
            a.x += 1;
            a.y += 1;
        } else if gravity.x == 1 {
            a.x += 1;
            a.y -= 1;
        } else if gravity.y == -1 {
            a.x -= 1;
            a.y -= 1;
        }
        if a.x > 0 && machine.velocity.x <= -3 {
            a.x = 0
        }
        if a.x < 0 && machine.velocity.x >= 3 {
            a.x = 0
        }
        if a.y > 0 && machine.velocity.y <= -3 {
            a.y = 0
        }
        if a.y < 0 && machine.velocity.y >= 3 {
            a.y = 0
        }
        if machine.position.x.abs() >= 100 {
            a.x = 0;
        }
        if machine.position.y.abs() >= 100 {
            a.y = 0;
        }
        if a != (Point { x: 0, y: 0 }) {
            commands.push(rust_game_base::Command::Thrust(machine.machine_id, a));
        }
        for opponent_machine in opponent_machines.iter() {
        }
    }
    commands
}
