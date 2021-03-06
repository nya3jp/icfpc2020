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
extern crate rand;
use rand::prelude::*;

use rust_game_base::Point;
use rust_game_base::actions;

fn main() {
    let resp = rust_game_base::send_join_request().unwrap();
    let start_params = initialize(&resp);
    let mut resp = rust_game_base::send_start_request(&start_params).unwrap();
    while resp.current_game_state != rust_game_base::CurrentGameState::END {
        let commands = play(&resp);
        resp = rust_game_base::send_command_request(&mut commands.into_iter()).unwrap();
    }
}

fn initialize(resp: &rust_game_base::Response) -> rust_game_base::Param {
    let total_cost = resp.stage_data.initialize_param.total_cost;
    let self_role = resp.stage_data.self_role;
    let mut start_params = rust_game_base::Param { energy: 0, laser_power: 0, cool_down_per_turn: 8, life: 1 };
    if self_role == rust_game_base::Role::ATTACKER {
        start_params.laser_power += 6;
    }
    start_params.energy = total_cost - 4 * start_params.laser_power - 12 * start_params.cool_down_per_turn - 2 * start_params.life;
    start_params
}

fn predict_crash_time(machine: &rust_game_base::Machine, obstacle: &rust_game_base::Obstacle) -> isize {
    let mut t = 0;
    let mut p = machine.position;
    let mut v = machine.velocity;
    while t < 32 {
        let a = actions::get_gravity_from_point(&p);
        v.y += a.y;
        v.x += a.x;
        p.y += v.y;
        p.x += v.x;
        if p.y.abs() <= obstacle.gravity_radius as isize {
            break;
        }
        if p.x.abs() <= obstacle.gravity_radius as isize {
            break;
        }
        if obstacle.stage_half_size as isize <= p.y.abs() {
            break;
        }
        if obstacle.stage_half_size as isize <= p.x.abs() {
            break;
        }
        t += 1;
    }
    t
}

fn play(resp: &rust_game_base::Response) -> Vec<rust_game_base::Command> {
    let mut commands = vec![];
    let current_state = resp.current_state.as_ref().unwrap();
    let obstacle = current_state.obstacle.unwrap();
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
        let mut a = Point { x: 0, y: 0 };
        let mut t = predict_crash_time(machine, &obstacle);
        let mut k = 1;
        if t < 10 {
            for dy in vec![-1, 0, 1] {
                for dx in vec![-1, 0, 1] {
                    let m = rust_game_base::Machine {
                        velocity: Point { y: machine.velocity.y + dy, x: machine.velocity.x + dx },
                        ..**machine
                    };
                    let dt = predict_crash_time(&m, &obstacle) - t;
                    if dt > 0 {
                        a = Point { y: dy, x: dx };
                        t += dt;
                        k = 1;
                    } else if dt == 0 {
                        k += 1;
                        if rand::random::<f64>() < 1.0 / (k as f64) {
                            a = Point { y: dy, x: dx };
                        }
                    }
                }
            }
        }
        if a != (Point { x: 0, y: 0 }) {
            commands.push(rust_game_base::Command::Thrust(machine.machine_id, Point { y: -a.y, x: -a.x }));
        }
    }
    for machine in self_machines.iter() {
        if machine.params.laser_power >= 1 && machine.heat <= machine.params.cool_down_per_turn {
            for opponent_machine in opponent_machines.iter() {
                let p = Point {
                    x: opponent_machine.position.x + opponent_machine.velocity.x,
                    y: opponent_machine.position.y + opponent_machine.velocity.y,
                };
                commands.push(rust_game_base::Command::Beam(machine.machine_id, p, machine.params.laser_power as isize));
                break
            }
        }
    }
    commands
}
