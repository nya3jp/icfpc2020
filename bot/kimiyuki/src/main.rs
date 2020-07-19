#![allow(non_snake_case, unused, non_upper_case_globals)]

extern crate rust_game_base;
#[macro_use]
extern crate itertools;

use rust_game_base::Point;

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
    if resp.current_state.as_ref().map_or(0, |current_state| current_state.turn) % 4 != 0 {
        let machines = resp.current_state.as_ref().map_or(vec![], |current_state| current_state.machines.clone());
        for (machine, _) in machines.iter() {
            if machine.role == resp.stage_data.self_role {
                let mut a = Point { x: 0, y: 0 };
                if (0 < machine.position.y && -4 <= machine.velocity.y) || (6 <= machine.velocity.y) {
                    a.y -= 1;
                }
                if (machine.position.y < 0 && machine.velocity.y <= 4) || (machine.velocity.y <= -6) {
                    a.y += 1;
                }
                if (0 < machine.position.x && -4 <= machine.velocity.x) || (6 <= machine.velocity.x) {
                    a.x -= 1;
                }
                if (machine.position.x < 0 && machine.velocity.x <= 4) || (machine.velocity.x <= -6) {
                    a.x += 1;
                }
                if a.y != 0 || a.x != 0 {
                    commands.push(rust_game_base::Command::Thrust(machine.machine_id, a));
                }
            }
        }
    }
    commands
}
