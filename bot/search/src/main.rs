#![allow(non_snake_case, unused, unused, non_upper_case_globals)]

extern crate rust_game_base;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;

use rust_game_base::*;

type Action = Vec<Command>;

fn main() {
    let resp = send_join_request().unwrap();
    let param = Solver::decide_param(&resp.stage_data);

    let my_role = resp.stage_data.self_role;

    let res = send_start_request(&param).unwrap();
    let mut state = res.current_state.unwrap();

    loop {
        let ab_result = alpha_beta(&state, my_role, 1, true);
    }
}

lazy_static! {
    static ref LIMIT: std::time::Duration = std::time::Duration::from_millis(150);
}

const BIG: f64 = 1e64;

#[derive(Debug)]
struct ABResult {
    // evaluation score of the current state.
    current_score: f64,
    attacker_action: Action,
    defender_action: Action,
}

fn win_result(winner: Role) -> ABResult {
    ABResult {
        current_score: if winner == Role::ATTACKER { BIG } else { -BIG },
        attacker_action: vec![],
        defender_action: vec![],
    }
}

impl ABResult {
    // is better than b for role.
    fn is_better_than_for(&self, b: &ABResult, role: Role) -> bool {
        if (role == Role::ATTACKER && self.current_score > b.current_score) {
            return true;
        }
        if (role == Role::DEFENDER && self.current_score < b.current_score) {
            return true;
        }
        return false;
    }

    fn set_action_for(&mut self, a: Action, role: Role) {
        if role == Role::ATTACKER {
            self.attacker_action = a;
        } else {
            self.defender_action = a;
        }
    }
}

// choose best move and next action.
fn alpha_beta(state: &CurrentState, my_role: Role, depth: usize, need_move: bool) -> ABResult {
    let your_role = my_role.opposite();

    if let Some(winner) = get_winner(&state) {
        return win_result(winner);
    }

    // if depth is 0, return evaluated score.
    if depth == 0 {
        let current_score = Solver::evaluate(&state);
        return ABResult {
            current_score,
            attacker_action: vec![],
            defender_action: vec![],
        };
    }

    // choose my move.
    // my worst result is your win.
    let mut my_best = win_result(my_role.opposite());

    for a in Solver::action_cands(state, my_role) {
        // choose your move.
        // your worst result is my win.
        let mut your_best = win_result(my_role);
        for b in Solver::action_cands(state, my_role.opposite()) {
            let mut v = a.clone();
            v.append(&mut b.clone());
            let (_, next_state) = state_update(state, &v);

            // evaluate next state
            let cur_res = alpha_beta(&next_state, my_role, depth - 1, false);

            if cur_res.is_better_than_for(&your_best, your_role) {
                your_best = cur_res;

                if need_move {
                    your_best.set_action_for(b.clone(), your_role);
                }
            }
        }

        // update my best move.
        if my_best.is_better_than_for(&your_best, my_role) {
            my_best = your_best;

            if need_move {
                my_best.set_action_for(a.clone(), my_role);
            }
        }
    }
    my_best
}

struct Solver {}

impl Solver {
    fn decide_param(static_info: &StageData) -> Param {
        Param {
            energy: 4,
            laser_power: 4,
            life: 4,
            cool_down_per_turn: 4,
        }
    }

    fn action_cands(state: &CurrentState, role: Role) -> Vec<Action> {
        let ids = get_roled_machine_ids(&state, role);

        let id = ids[0];

        let mut res = vec![];

        res.push(vec![]); // do nothing

        const max_len: usize = 10;
        res.push(vec![Command::Bomb(id)]);
        for (dx, dy) in iproduct!(-1..=1, -1..=1) {
            if dx == 0 && dy == 0 {
                continue;
            }
            res.push(vec![Command::Thrust(id, Point::new(dx, dy))]);
        }
        res
    }

    // evaluate state.
    // 大きいと，attacker が有利であることを示す．
    fn evaluate(state: &CurrentState) -> f64 {
        0.0
    }
}
