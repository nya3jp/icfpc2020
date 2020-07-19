use self::super::game::*;
use std::cmp::{max, min};

const THRUST_HEAT: usize = 8;
const LASER_HEAT: usize = 64;
const OVERHEAT: usize = 64;
const THRUST_ENERGY: usize = 1;

fn machine_generated_heat(m: &Machine, heat: usize) -> Machine {
    Machine {
        generated_heat: m.generated_heat + heat,
        ..*m
    }
}

fn machine_damage(m: &Machine, damage: usize) -> Machine {
    Machine {
        attack_heat: m.attack_heat + damage,
        ..*m
    }
}

// returns None if machines die
fn machine_cooldown(m: &Machine) -> Machine {
    let energy = m.params.energy;
    let newh = m.heat + m.attack_heat;
    let newh = newh - min(m.params.cool_down_per_turn, newh);
    println!("newh: {}", newh);
    if ((newh > OVERHEAT) && // attack indeed does damage and kill
        (energy < (newh - OVERHEAT)))
    {
        // "0 0 0 0"
        return Machine {
            params: Param {
                energy: 0,
                laser_power: 0,
                cool_down_per_turn: 0,
                life: 0,
            },
            ..*m
        };
    };
    // otherwise, cooldown is first used to block attack heat and then remaining heat deletes their own energy / laser effectiveness

    let newheat = m.heat + m.attack_heat + m.generated_heat - m.params.cool_down_per_turn;
    let heatdamage = newheat - min(newheat, OVERHEAT);

    let newheat = min(newheat, OVERHEAT);
    let heatdamage = max(heatdamage, 0);

    let energydamage = min(m.params.energy, heatdamage);
    let laserdamage = min(heatdamage - energydamage, m.params.laser_power); // TODO (coner case): what if laser damage exceeds remaining laser eff?

    Machine {
        params: Param {
            energy: m.params.energy - energydamage,
            laser_power: m.params.laser_power - laserdamage,
            ..m.params
        },
        heat: newheat,
        generated_heat: 0,
        attack_heat: 0,
        ..*m
    }
}

fn state_cooldown(s: &CurrentState) -> CurrentState {
    let mut ret = s.clone();
    let machines = &s.machines;
    let retmachine = machines
        .iter()
        .map(|(m, a)| (machine_cooldown(m), a.clone()))
        .collect();
    ret.machines = retmachine;
    ret
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
    let damage = max(damage_base * power - diminish, 0);
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
        mpair.0 = machine_damage(&mpair.0, finaldamage); // FIXME TODO: ActionResults
    }
}

fn do_laser(s: &CurrentState, all_actions: &Vec<Command>) -> CurrentState {
    let mut newstate = s.clone();
    for (i, action) in all_actions.iter().enumerate() {
        match action {
            Command::Beam(shipnum, pt, power) => {
                do_laser_helper(&mut newstate, *shipnum, &pt, *power as isize);
            }
            _ => (),
        };
    }
    newstate
}

fn state_update_damages(cstate: &CurrentState, commands: &Vec<Command>) -> CurrentState {
    let cstate = do_laser(cstate, commands);
    // TODO jibaku
    cstate
}

fn state_update_velocities(cstate: &CurrentState, commands: &Vec<Command>) -> CurrentState {
    let mut ncount = 0;
    let mut newstate = cstate.clone();
    for c in commands {
        match c {
            Command::Thrust(shipnum, delta) => {
                if (delta.l0_distance() == 0) {
                    panic!("Thrust(0,0) cannot be chosen in alien GUI")
                };
                for (i, (m, actionresult)) in cstate.machines.iter().enumerate() {
                    if m.machine_id != (*shipnum as isize) {
                        continue;
                    } else if m.params.energy < THRUST_ENERGY {
                        // energy check
                        // can't thrust
                        continue;
                    } else {
                        let newmachine = machine_generated_heat(m, THRUST_HEAT);
                        let newmachine = Machine {
                            velocity: newmachine.velocity - *delta, // Thrusts to inverse direction
                            params: Param {
                                energy: newmachine.params.energy - THRUST_ENERGY,
                                ..newmachine.params
                            },
                            ..newmachine
                        };
                        newstate.machines[i] =
                            (newmachine, vec![ActionResult::Thruster { a: *delta }]);
                    }
                }
                ncount += 1
            }
            _ => (),
        }
    }
    if ncount >= 2 {
        panic!("Multiple thrusts in one action")
    }
    newstate
}

fn state_update_coordinates(s: &CurrentState) -> CurrentState {
    let mut newmachines = s.machines.clone();
    for mut mp in &mut newmachines {
        let m = &mut mp.0;
        m.position = m.position + m.velocity
    }
    CurrentState {
        machines: newmachines,
        ..*s
    }
}

fn state_clear_actions(cstate: &CurrentState) -> CurrentState {
    let newmachines = cstate
        .machines
        .iter()
        .map(|(x, _)| (x.clone(), vec![]))
        .collect();
    CurrentState {
        machines: newmachines,
        ..*cstate
    }
}

pub fn state_update(cstate: &CurrentState, commands: &Vec<Command>) -> CurrentState {
    let cstate = state_clear_actions(cstate);
    let cstate = state_update_velocities(&cstate, commands);
    let cstate = state_update_coordinates(&cstate);
    let cstate = state_update_damages(&cstate, commands);
    let cstate = state_cooldown(&cstate);
    cstate
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
            _1: 64,
            _2: 1,
            generated_heat: 0,
            attack_heat: 0,
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
            _1: 64,
            _2: 1,
            generated_heat: 0,
            attack_heat: 0,
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
        let updated = state_update(&curstate, &vec![cmd1]);
        //println!("{:?}", updated);
        // machine 1 should be unchanged
        assert_eq!(updated.machines[0].0.position, Point { x: 33, y: 6 });
        assert_eq!(updated.machines[0].0.velocity, Point { x: 0, y: 0 });
        // machine 2 should ...
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
