use crate::game::*;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

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
    get_gravity_from_point(&pos)
}

pub fn get_gravity_from_point(pos: &Point) -> Point {
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

// Assuming the current velocity satisfies |vx| <= 1, |vy| <= 1,
// cooldown_per_turn >= 8, and it has enough energy, returns the
// next command to get to the target.
// TODO: consider the planet.
pub fn move_to(state: &CurrentState, machine_id: isize, target: Point) -> Option<Command> {
    let machine = get_machine_by_id(state, machine_id).unwrap();
    let d = target - machine.position;
    // let target_v = Point {x: d.x.signum(), y: d.y.signum()};
    let target_v = if d.x.abs() < d.y.abs() {
        Point {
            x: 0,
            y: d.y.signum(),
        }
    } else {
        Point {
            x: d.x.signum(),
            y: 0,
        }
    };
    eprintln!("target v: {:?}", target_v);
    let dv = target_v - machine.velocity - get_gravity(state, machine_id);
    if dv == (Point { x: 0, y: 0 }) {
        return None;
    }
    return Some(Command::Thrust(machine_id, -dv));
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct BfsState {
    position: Point,
    velocity: Point,
}

impl BfsState {
    const fn new(position: Point, velocity: Point) -> Self {
        Self { position, velocity }
    }
}

pub fn move_to2(state: &CurrentState, machine_id: isize, target: Point) -> Option<Command> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    let machine = get_machine_by_id(state, machine_id).unwrap();
    let init = BfsState::new(machine.position, machine.velocity);
    visited.insert(init.clone(), None);
    queue.push_back(init.clone());

    let gravity_radius = if let Some(obstacle) = state.obstacle {
        obstacle.gravity_radius as isize
    } else {
        0
    };

    let result = 'search: loop {
        if queue.is_empty() {
            break None;
        }
        let top = queue.pop_front().unwrap();
        let gravity = if state.obstacle.is_none() {
            Point { x: 0, y: 0 }
        } else {
            get_gravity_from_point(&top.position)
        };
        for nvy in -1..=1 {
            let ay = nvy - top.velocity.y - gravity.y;
            if ay.abs() > 1 {
                continue;
            }
            let ny = top.position.y + nvy;
            if ny.abs() < gravity_radius {
                continue;
            }
            for nvx in -1..=1 {
                let ax = nvx - top.velocity.x - gravity.x;
                if ax.abs() > 1 {
                    continue;
                }
                let nx = top.position.x + nvx;
                if nx.abs() < gravity_radius {
                    continue;
                }

                let np = Point::new(nx, ny);
                let mut inserted = false;
                let mut insert = || {
                    inserted = true;
                    Some(top.clone())
                };
                visited
                    .entry(BfsState::new(np, Point::new(nvx, nvy)))
                    .or_insert_with(insert);
                if inserted {
                    if np == target {
                        break 'search Some(BfsState::new(np, Point::new(nvx, nvy)));
                    }
                    queue.push_back(BfsState::new(np, Point::new(nvx, nvy)))
                }
            }
        }
    };

    if result.is_none() {
        // Failed to find a path.
        return None;
    }

    // Back track.
    let mut next = result.unwrap();
    loop {
        let cur = visited.get(&next);
        if cur.is_none() {
            // Should not happen.
            return None;
        }
        let cur = cur.unwrap();
        if cur.is_none() {
            // Should not happen.
            return None;
        }
        let cur = cur.as_ref().unwrap();
        if cur == &init {
            let dv = next.velocity - cur.velocity - get_gravity_from_point(&cur.position);
            return Some(Command::Thrust(machine_id, -dv));
        }
        next = cur.clone();
    }
}

#[derive(Eq, PartialEq)]
struct PState {
    state: BfsState,
    prev: Option<BfsState>,
    g_cost: usize,
    f_cost: usize,
}

impl PState {
    const fn new(state: BfsState, prev: Option<BfsState>, g_cost: usize, f_cost: usize) -> Self {
        Self {
            state,
            prev,
            g_cost,
            f_cost,
        }
    }
}

// To make a min-heap.
impl Ord for PState {
    fn cmp(&self, other: &PState) -> Ordering {
        other.f_cost.cmp(&self.f_cost).then_with(|| {
            (
                self.state.position.x,
                self.state.position.y,
                self.state.velocity.x,
                self.state.velocity.y,
            )
                .cmp(&(
                    other.state.position.x,
                    other.state.position.y,
                    other.state.velocity.x,
                    other.state.velocity.y,
                ))
        })
    }
}

impl PartialOrd for PState {
    fn partial_cmp(&self, other: &PState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn linf_dist(p1: Point, p2: Point) -> usize {
    return cmp::max((p1.x - p2.x).abs(), (p1.y - p2.y).abs()) as usize;
}

pub fn move_to3(state: &CurrentState, machine_id: isize, target: Point) -> Option<Command> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    let machine = get_machine_by_id(state, machine_id).unwrap();
    let init = BfsState::new(machine.position, machine.velocity);
    queue.push_back(PState::new(
        init.clone(),
        None,
        0,
        linf_dist(machine.position, target),
    ));

    let gravity_radius = if let Some(obstacle) = state.obstacle {
        obstacle.gravity_radius as isize
    } else {
        0
    };

    let result = 'search: loop {
        if queue.is_empty() {
            break None;
        }
        let top = queue.pop_front().unwrap();
        let mut inserted = false;
        let mut insert = || {
            inserted = true;
            top.prev.clone()
        };
        visited.entry(top.state.clone()).or_insert_with(insert);
        if !inserted {
            continue;
        }
        if top.state.position == target {
            break 'search Some(top.state);
        }

        let gravity = if state.obstacle.is_none() {
            Point { x: 0, y: 0 }
        } else {
            get_gravity_from_point(&top.state.position)
        };
        for nvy in -1..=1 {
            let ay = nvy - top.state.velocity.y - gravity.y;
            if ay.abs() > 1 {
                continue;
            }
            let ny = top.state.position.y + nvy;
            if ny.abs() < gravity_radius {
                continue;
            }
            for nvx in -1..=1 {
                let ax = nvx - top.state.velocity.x - gravity.x;
                if ax.abs() > 1 {
                    continue;
                }
                let nx = top.state.position.x + nvx;
                if nx.abs() < gravity_radius {
                    continue;
                }

                let np = Point::new(nx, ny);
                queue.push_back(PState::new(
                    BfsState::new(np, Point::new(nvx, nvy)),
                    Some(top.state.clone()),
                    top.g_cost + 1,
                    top.g_cost + 1 + linf_dist(np, target),
                ));
            }
        }
    };

    if result.is_none() {
        // Failed to find a path.
        return None;
    }

    // Back track.
    let mut next = result.unwrap();
    loop {
        let cur = visited.get(&next);
        if cur.is_none() {
            // Should not happen.
            return None;
        }
        let cur = cur.unwrap();
        if cur.is_none() {
            // Should not happen.
            return None;
        }
        let cur = cur.as_ref().unwrap();
        if cur == &init {
            let dv = next.velocity - cur.velocity - get_gravity_from_point(&cur.position);
            return Some(Command::Thrust(machine_id, -dv));
        }
        next = cur.clone();
    }
}
