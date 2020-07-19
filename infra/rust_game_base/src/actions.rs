use crate::game::*;

// Assuming the current velocity is (0, 0), cool_down_per_turn >= 8,
// and it has enough energy, returns the command to stay at the current
// position.
pub fn stay(state: &CurrentState, machine_id: isize) -> Option<Command> {
    if state.obstacle.is_none() {
        return None;
    }

    let gravity = get_gravity(state, machine_id);
    if gravity == (Point { x: 0, y: 0 }) {
        return None;
    }
    return Some(Command::Thrust(machine_id, gravity));
}

// Returns the gravity.
pub fn get_gravity(state: &CurrentState, machine_id: isize) -> Point {
    if state.obstacle.is_none() {
        return Point { x: 0, y: 0 };
    }

    let pos = state
        .machines
        .iter()
        .find(|(x, _)| x.machine_id == machine_id)
        .unwrap()
        .0
        .position;
    Point {
        x: if pos.x.abs() < pos.y.abs() {
            0
        } else {
            -pos.x.signum()
        },
        y: if pos.y.abs() < pos.x.abs() {
            0
        } else {
            -pos.y.signum()
        },
    }
}
