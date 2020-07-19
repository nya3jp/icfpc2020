use self::super::game::*;

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

fn machine_update_velocity(cstate: &CurrentState, commands: &Vec<Command>) -> CurrentState {
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

fn machine_update_coordinate(m: &Machine) -> Machine {
    Machine {
        position: m.position + m.velocity,
        ..*m
    }
}

fn machine_damage(m: &Machine, damage: usize) -> Machine {
    Machine {
        attack_heat: m.attack_heat + damage,
        ..*m
    }
}
