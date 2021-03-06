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

use self::super::game::*;
use std::cmp::{max, min};

const THRUST_HEAT: usize = 8;
const OVERHEAT: usize = 64;
const THRUST_ENERGY: usize = 1;

fn machine_generated_heat(m: &Machine, heat: usize) -> Machine {
    Machine {
        heat: m.heat + heat,
        ..*m
    }
}

fn machine_damage(m: &Machine, damage: usize) -> Machine {
    Machine {
        heat: m.heat + damage,
        ..*m
    }
}

fn update_machine_heat(m: &mut Machine, heat: usize) {
    m.heat += heat
}

// returns None if machines die
fn machine_update_cooldown(m: &mut Machine) {
    let energy = m.params.energy as isize;
    let newh = m.heat as isize - min(m.params.cool_down_per_turn, m.heat) as isize;

    let heatdamage = newh - min(newh, m.heat_limit as isize) as isize;
    let newheat = min(newh, m.heat_limit as isize) as isize;
    m.heat = newheat as usize;

    let remaindamage = max(heatdamage, 0) as isize;
    let energydamage = min(m.params.energy as isize, remaindamage);
    m.params.energy -= energydamage as usize;

    let remaindamage = max(remaindamage - energydamage, 0);
    let laserdamage = min(remaindamage, m.params.laser_power as isize);
    m.params.laser_power -= laserdamage as usize;

    let remaindamage = max(remaindamage - laserdamage, 0);
    let cooldowndamage = min(remaindamage, m.params.cool_down_per_turn as isize);
    m.params.cool_down_per_turn -= cooldowndamage as usize;

    let remaindamage = max(remaindamage - cooldowndamage, 0);
    let lifedamage = min(remaindamage, m.params.life as isize);
    m.params.life -= lifedamage as usize;
}

fn state_update_cooldown(s: &mut CurrentState) {
    for m in &mut s.machines {
        machine_update_cooldown(&mut m.0)
    }
}

fn lookup_machine(s: &CurrentState, id: isize) -> Option<Machine> {
    for (m, _) in &s.machines {
        if m.machine_id == (id as isize) {
            return Some(*m);
        }
    }
    return None;
}

fn laser_damage_base(delta: &Point) -> isize {
    // FIXME need experiments
    if (delta.x == 0 || delta.y == 0) {
        3
    } else if (delta.x == delta.y || delta.x == -delta.y) {
        3
    } else if (delta.x == delta.y * 2
        || delta.x == -delta.y * 2
        || delta.x * 2 == delta.y
        || -delta.x * 2 == delta.y)
    {
        0
    } else if (delta.x == delta.y * 3
        || delta.x == -delta.y * 3
        || delta.x * 3 == delta.y
        || -delta.x * 3 == delta.y)
    {
        2
    } else if (delta.x * 4 == delta.y * 3
        || delta.x * 4 == -delta.y * 3
        || delta.x * 3 == delta.y * 4
        || -delta.x * 3 == delta.y * 4)
    {
        2
    } else {
        1
    }
}

fn do_laser_helper(s: &mut CurrentState, shipnum: isize, target: &Point, power: isize) {
    let origin = lookup_machine(s, shipnum).unwrap();
    let dx = *target - origin.position;
    let damage_base = laser_damage_base(&dx);
    let diminish = dx.lmax_distance() - 1;
    let damage = max(damage_base * power - diminish, 0); // should be OK because it's isize
    for mpair in &mut s.machines {
        // check position
        let m = &mut mpair.0;
        let dpos = m.position - *target;
        let dist = dpos.lmax_distance();
        let finaldamage = if dist > 15 {
            0
        } else {
            damage as usize >> (2 * dist as usize)
        };
        mpair.0.heat += finaldamage as usize;

        if (mpair.0.machine_id == shipnum) {
            mpair.0.heat += power as usize; // self heat dmg
            mpair.1.push(ActionResult::Laser {
                opponent: *target,
                power: power as usize,
                intensity: 0, // FIXME TODO.
                _3: 0,        // FIXME TODO.
            }) // FIXME TODO: damage
        }
    }
}

fn do_laser(s: &mut CurrentState, all_actions: &Vec<Command>) {
    for action in all_actions {
        match action {
            Command::Beam(shipnum, pt, power) => {
                do_laser_helper(s, *shipnum, &pt, *power as isize);
            }
            _ => (),
        };
    }
}

fn do_self_destruct_helper(
    s: &mut CurrentState,
    shipnum: isize,
    size: usize,
    power: usize,
    attackorigin: Point,
) {
    for mpair in &mut s.machines {
        let distance = (mpair.0.position - attackorigin).lmax_distance();
        if distance as usize <= size {
            // TODO: Verify this.
            let diminish = 32 * (distance as usize);
            update_machine_heat(&mut mpair.0, power - min(power, diminish));
        }
        if shipnum == mpair.0.machine_id {
            // self destruct
            mpair.0.params = Param {
                energy: 0,
                laser_power: 0,
                cool_down_per_turn: 0,
                life: 0,
            };
            mpair.1.push(ActionResult::Bomb {
                power: power,
                area: size,
            });
        }
    }
}

const SELF_DESTRUCT_TABLE: [(usize, usize); 35] = [
    (1, 128),
    (2, 161),
    (3, 181),
    (4, 195),
    (5, 205),
    (6, 214),
    (8, 227),
    (9, 233),
    (10, 238),
    (12, 246),
    (15, 256),
    (16, 258),
    (17, 261),
    (18, 263),
    (19, 266),
    (24, 275),
    (32, 287),
    (33, 288),
    (40, 296),
    (48, 303),
    (56, 309),
    (64, 314),
    (65, 314),
    (72, 318),
    (80, 322),
    (96, 328),
    (100, 330),
    (110, 333),
    (128, 338),
    (178, 350),
    (323, 369),
    (333, 370),
    (343, 371),
    (380, 374),
    (384, 375),
];

fn lookup_destruct_power_table(sumenergy: usize) -> usize {
    let pos = SELF_DESTRUCT_TABLE.binary_search_by_key(&sumenergy, |&(a, b)| a);
    match pos {
        Ok(index) => SELF_DESTRUCT_TABLE[index].1,
        Err(index) => {
            if index >= SELF_DESTRUCT_TABLE.len() {
                return SELF_DESTRUCT_TABLE.last().unwrap().1;
            } else if index == 0 {
                return 0;
            }
            let left = SELF_DESTRUCT_TABLE[index - 1];
            let right = SELF_DESTRUCT_TABLE[index];
            let dx = right.0 - left.0;
            let dy = right.1 - left.1;
            left.1 + (sumenergy - left.0) * dy / dx
        }
    }
}

fn self_destruct_power(m: &Machine) -> (usize, usize) {
    let area = 32;
    let sumenergy =
        m.params.energy + m.params.laser_power + m.params.cool_down_per_turn + m.params.life;
    (area, lookup_destruct_power_table(sumenergy))
}

pub fn self_destruct_damage(m: &Machine, target: Point) -> usize {
    let (_, power) = self_destruct_power(&m);
    let d = (target - m.position + m.velocity).lmax_distance();
    let damage = power as isize - d * 32;
    if damage < 0 {
        0
    } else {
        damage as usize
    }
}

fn do_self_destruct(s: &mut CurrentState, all_actions: &Vec<Command>) {
    for action in all_actions {
        match action {
            Command::Bomb(shipnum) => {
                let origin = lookup_machine(s, *shipnum).unwrap();
                let (size, power) = self_destruct_power(&origin);
                do_self_destruct_helper(s, *shipnum, size, power, origin.position);
            }
            _ => (),
        }
    }
}

fn state_update_damages(cstate: &mut CurrentState, commands: &Vec<Command>) {
    do_laser(cstate, commands);
    do_self_destruct(cstate, commands);
}

fn state_update_velocities(cstate: &mut CurrentState, commands: &Vec<Command>) {
    let mut ncount = 0;
    let mut thrust_ids = Vec::new();
    for c in commands {
        match c {
            Command::Thrust(shipnum, delta) => {
                if (delta.lmax_distance() == 0) {
                    panic!("Thrust(0,0) cannot be chosen in alien GUI")
                };
                if thrust_ids.iter().any(|x| *x == shipnum) {
                    panic!("Multiple thrusts from same id");
                };
                thrust_ids.push(shipnum);
                for (m, actionresult) in &mut cstate.machines {
                    if m.machine_id != (*shipnum as isize) {
                        continue;
                    } else if m.params.energy < THRUST_ENERGY {
                        // energy check
                        // can't thrust
                        continue;
                    } else {
                        m.heat += THRUST_HEAT;
                        m.velocity = m.velocity - *delta;
                        m.params.energy = m.params.energy - THRUST_ENERGY;
                        actionresult.push(ActionResult::Thruster { a: *delta });
                    }
                }
            }
            _ => (),
        }
    }
}

fn state_update_coordinates(s: &mut CurrentState) {
    for mut mp in &mut s.machines {
        let m = &mut mp.0;
        m.position = m.position + m.velocity
    }
}

fn state_update_kill_gravity(cstate: &mut CurrentState) {
    match cstate.obstacle {
        None => (),
        Some(obs) => {
            for m in &mut cstate.machines {
                let pos = m.0.position;
                if pos.lmax_distance() <= obs.gravity_radius as isize {
                    // kill
                    m.0.params = Param {
                        energy: 0,
                        laser_power: 0,
                        cool_down_per_turn: 0,
                        life: 0,
                    }
                }
            }
        }
    }
}

fn state_clone_clear_actions(cstate: &CurrentState) -> CurrentState {
    let mut newstate = cstate.clone();
    for m in &mut newstate.machines {
        m.1.clear()
    }
    newstate
}

fn state_update_obstacles(cstate: &mut CurrentState) {
    match cstate.obstacle {
        None => (),
        Some(obs) => {
            for m in &mut cstate.machines {
                let x = m.0.position.x;
                let y = m.0.position.y;
                let mut fx = 0;
                let mut fy = 0;
                if x > 0 && x.abs() >= y.abs() {
                    fx -= 1;
                }
                if x < 0 && x.abs() >= y.abs() {
                    fx += 1;
                }
                if y > 0 && x.abs() <= y.abs() {
                    fy -= 1;
                }
                if y < 0 && x.abs() <= y.abs() {
                    fy += 1;
                }
                m.0.velocity = m.0.velocity + Point { x: fx, y: fy }
            }
        }
    }
}

fn is_dead(m: &Machine) -> bool {
    return m.params
        == Param {
            energy: 0,
            laser_power: 0,
            cool_down_per_turn: 0,
            life: 0,
        };
}

fn get_current_gamestate(cstate: &CurrentState) -> (CurrentGameState, Option<Role>) {
    let mut defender_alive = false;
    let mut attacker_alive = false;
    for mpair in &cstate.machines {
        let m_is_dead = is_dead(&mpair.0);
        if !m_is_dead {
            match mpair.0.role {
                Role::ATTACKER => attacker_alive = true,
                Role::DEFENDER => defender_alive = true,
            }
        }
    }
    // TODO: winner is?
    if (!defender_alive) || (!attacker_alive) {
        (
            CurrentGameState::END,
            Some(if defender_alive {
                Role::DEFENDER
            } else {
                Role::ATTACKER
            }),
        )
    } else {
        (CurrentGameState::PLAYING, None)
    }
}

pub fn get_winner(cstate: &CurrentState) -> Option<Role> {
    get_current_gamestate(cstate).1
}

/* Accepts CurrentState and Commands and outputs updated states. */
pub fn state_update(
    cstate: &CurrentState,
    commands: &Vec<Command>,
) -> (CurrentGameState, CurrentState) {
    let mut cstate = state_clone_clear_actions(cstate);
    state_update_obstacles(&mut cstate);
    state_update_velocities(&mut cstate, commands);
    state_update_coordinates(&mut cstate);
    state_update_damages(&mut cstate, commands);
    state_update_cooldown(&mut cstate);
    state_update_kill_gravity(&mut cstate);
    (get_current_gamestate(&cstate).0, cstate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let curstate = {
            let machine1 = Machine {
                role: Role::DEFENDER,
                machine_id: 1,
                position: Point { x: 33, y: 6 },
                velocity: Point { x: 0, y: 0 },
                params: Param {
                    energy: 78,
                    laser_power: 0,
                    cool_down_per_turn: 0,
                    life: 1,
                },
                heat: 0,
                heat_limit: 64,
                move_limit: 1,
            };
            let machine2 = Machine {
                role: Role::ATTACKER,
                machine_id: 0,
                position: Point { x: 20, y: 0 },
                velocity: Point { x: 1, y: 0 },
                params: Param {
                    energy: 2,
                    laser_power: 0,
                    cool_down_per_turn: 0,
                    life: 1,
                },
                heat: 8,
                heat_limit: 64,
                move_limit: 1,
            };
            CurrentState {
                turn: 0,
                obstacle: None,
                machines: vec![(machine1, vec![]), (machine2, vec![])],
            }
        };
        //println!("{:?}", curstate);
        let cmd1 = Command::Thrust(0, Point { x: -1, y: 0 });
        let (status, updated) = state_update(&curstate, &vec![cmd1]);
        //println!("{:?}", updated);
        // machine 1 should be unchanged
        assert_eq!(updated.machines[0].0.position, Point { x: 33, y: 6 });
        assert_eq!(updated.machines[0].0.velocity, Point { x: 0, y: 0 });
        // machine 2 should ...
        assert_eq!(status, CurrentGameState::PLAYING);
        assert_eq!(updated.machines[1].0.position, Point { x: 22, y: 0 });
        assert_eq!(updated.machines[1].0.velocity, Point { x: 2, y: 0 });

        assert_eq!(
            updated.machines[1].0.params,
            Param {
                energy: 1,
                laser_power: 0,
                cool_down_per_turn: 0,
                life: 1
            }
        );
        assert_eq!(updated.machines[1].0.heat, 16);
    }

    #[test]
    fn test_laser_overheat() {
        let curstate = {
            let machine1 = Machine {
                role: Role::DEFENDER,
                machine_id: 1,
                position: Point { x: 30, y: -6 },
                velocity: Point { x: -2, y: -3 },
                params: Param {
                    energy: 291,
                    laser_power: 0,
                    cool_down_per_turn: 7,
                    life: 1,
                },
                heat: 0,
                heat_limit: 64,
                move_limit: 1,
            };
            let machine2 = Machine {
                role: Role::ATTACKER,
                machine_id: 0,
                position: Point { x: -27, y: -5 },
                velocity: Point { x: 3, y: -2 },
                params: Param {
                    energy: 18,
                    laser_power: 64,
                    cool_down_per_turn: 10,
                    life: 1,
                },
                heat: 64,
                heat_limit: 64,
                move_limit: 1,
            };
            CurrentState {
                turn: 0,
                obstacle: Some(Obstacle {
                    gravity_radius: 16,
                    stage_half_size: 128,
                }),
                machines: vec![(machine1, vec![]), (machine2, vec![])],
            }
        };

        //println!("{:?}", curstate);
        // step 1
        let (status, updated) = state_update(
            &curstate,
            &vec![Command::Beam(0, Point { x: -20, y: 40 }, 64)],
        );
        //println!("{:?}", updated);
        // machine 1 position
        assert_eq!(updated.machines[0].0.position, Point { x: 27, y: -9 });
        assert_eq!(updated.machines[0].0.velocity, Point { x: -3, y: -3 });
        // machine 2 should ...
        assert_eq!(status, CurrentGameState::PLAYING);
        assert_eq!(updated.machines[1].0.position, Point { x: -23, y: -7 });
        assert_eq!(updated.machines[1].0.velocity, Point { x: 4, y: -2 });
        assert_eq!(
            updated.machines[1].0.params,
            Param {
                energy: 0,
                laser_power: 28,
                cool_down_per_turn: 10,
                life: 1
            }
        );
        assert_eq!(updated.machines[1].0.heat, 64);

        // step 2
        let (status, updated) = state_update(
            &updated,
            &vec![Command::Beam(0, Point { x: -34, y: 28 }, 28)],
        );
        // machine 1 position
        assert_eq!(updated.machines[0].0.position, Point { x: 23, y: -12 });
        assert_eq!(updated.machines[0].0.velocity, Point { x: -4, y: -3 });
        // machine 2 should ...
        assert_eq!(status, CurrentGameState::PLAYING);
        assert_eq!(updated.machines[1].0.position, Point { x: -18, y: -9 });
        assert_eq!(updated.machines[1].0.velocity, Point { x: 5, y: -2 });
        assert_eq!(
            updated.machines[1].0.params,
            Param {
                energy: 0,
                laser_power: 10,
                cool_down_per_turn: 10,
                life: 1
            }
        );
        assert_eq!(updated.machines[1].0.heat, 64);
    }

    #[test]
    fn test_laser_kill() {
        let curstate = {
            let machine1 = Machine {
                role: Role::DEFENDER,
                machine_id: 19,
                position: Point { x: 35, y: -5 },
                velocity: Point { x: 2, y: 4 },
                params: Param {
                    energy: 0,
                    laser_power: 0,
                    cool_down_per_turn: 0,
                    life: 1,
                },
                heat: 90,
                heat_limit: 128,
                move_limit: 2,
            };
            let machine2 = Machine {
                role: Role::ATTACKER,
                machine_id: 1,
                position: Point { x: 13, y: 25 },
                velocity: Point { x: 8, y: -1 },
                params: Param {
                    energy: 27,
                    laser_power: 96,
                    cool_down_per_turn: 8,
                    life: 1,
                },
                heat: 40,
                heat_limit: 128,
                move_limit: 2,
            };
            let machine3 = Machine {
                // to prevent game over
                role: Role::DEFENDER,
                machine_id: 0,
                position: Point { x: 30, y: -62 },
                velocity: Point { x: 3, y: 9 },
                params: Param {
                    energy: 3,
                    laser_power: 0,
                    cool_down_per_turn: 8,
                    life: 4,
                },
                heat: 0,
                heat_limit: 128,
                move_limit: 2,
            };
            CurrentState {
                turn: 0,
                obstacle: Some(Obstacle {
                    gravity_radius: 12,
                    stage_half_size: 128,
                }),
                machines: vec![(machine1, vec![]), (machine2, vec![]), (machine3, vec![])],
            }
        };
        // step 1
        let (status, updated) = state_update(
            &curstate,
            &vec![Command::Beam(1, Point { x: -36, y: -1 }, 88)],
        );
        //println!("{:?}", updated);
        // machine 1 position
        assert_eq!(updated.machines[0].0.velocity, Point { x: 1, y: 4 });
        assert_eq!(updated.machines[0].0.position, Point { x: 36, y: -1 });
        // machine 2 should ...
        assert_eq!(status, CurrentGameState::PLAYING);
        assert_eq!(updated.machines[1].0.position, Point { x: 21, y: 23 });
        assert_eq!(updated.machines[1].0.velocity, Point { x: 8, y: -2 });
        assert_eq!(
            updated.machines[1].0.params,
            Param {
                energy: 27,
                laser_power: 96,
                cool_down_per_turn: 8,
                life: 1
            }
        );
        assert_eq!(updated.machines[1].0.heat, 120);
    }

    #[test]
    fn power_table_test() {
        assert_eq!(lookup_destruct_power_table(1), 128);
        assert_eq!(lookup_destruct_power_table(2), 161);
        assert_eq!(lookup_destruct_power_table(3), 181);
        assert!(lookup_destruct_power_table(11) > 238);
        assert!(lookup_destruct_power_table(11) < 246);
        assert_eq!(lookup_destruct_power_table(384), 375);
        assert!(lookup_destruct_power_table(385) >= 375);
        assert!(lookup_destruct_power_table(0) < 128);
    }
}
