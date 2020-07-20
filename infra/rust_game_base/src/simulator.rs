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
    let diminish = dx.l0_distance() - 1;
    let damage = max(damage_base * power - diminish, 0); // should be OK because it's isize
    for mpair in &mut s.machines {
        // check position
        let m = &mut mpair.0;
        let dpos = m.position - *target;
        let dist = dpos.l0_distance();
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
        let distance = (mpair.0.position - attackorigin).l0_distance();
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

fn self_destruct_power(m: &Machine) -> (usize, usize) {
    let sumenergy =
        m.params.energy + m.params.laser_power + m.params.cool_down_per_turn + m.params.life;
    if sumenergy <= 1 {
        (9, 128)
    } else if sumenergy <= 2 {
        (11, 161)
    } else if sumenergy <= 3 {
        (11, 181)
    } else if sumenergy <= 15 {
        (17, 256)
    } else {
        // sumenergy <= 511 ?
        (25, 384)
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
                if (delta.l0_distance() == 0) {
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
                if y > 0 && x.abs() <= y.abs() {
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
    (get_current_gamestate(&cstate).0, cstate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test_state() -> CurrentState {
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
    }

    #[test]
    fn test_update() {
        let curstate = init_test_state();
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
}
