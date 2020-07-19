#![allow(non_snake_case, unused, non_upper_case_globals)]

extern crate rust_game_base;
#[macro_use]
extern crate itertools;

/*
仕様があまり定まっていないので，generic な感じの AI を書く．

- 攻撃側であることを仮定．
- 分裂はしないことにする (自機はつねにひとつ．)．
- 相手は次の段階では動かないことを仮定．
*/

fn main() {
    Proxy::join_game();
    let state = Proxy::start_game(&StartParam {});
    run(state);
}

struct Proxy;

impl Proxy {
    fn join_game() {
        // TODO
        // rust_game_base::send_join_request();
    }
    fn start_game(p: &StartParam) -> State {
        // TODO
        // rust_game_base::send_start_request(4, 4, 4, 4)

        State::mock()
    }

    fn do_action(a: &Action) -> State {
        // TODO

        State::mock()
    }
}

struct StartParam {}

fn run(mut state: State) {
    loop {
        let mut best = (-10000.0, vec![]);
        for a in possible_actions(&state) {
            let ns = next_state(&state, &a);
            let val = evaluate_state(&ns);
            if best.0 < val {
                best = (val, a);
            }
        }
        state = Proxy::do_action(&best.1);
        if state.finished() {
            return;
        }
    }
}

#[derive(Clone, Debug, Copy)]
struct P {
    x: isize,
    y: isize,
}

impl P {
    fn new(x: isize, y: isize) -> P {
        P { x, y }
    }
    fn dist(&self, p: P) -> f64 {
        self.dist2(p).sqrt()
    }
    fn dist2(&self, p: P) -> f64 {
        self.sub(p).norm2()
    }
    fn norm2(&self) -> f64 {
        let (x, y) = (self.x, self.y);
        (x * x + y * y) as _
    }
    fn sub(&self, p: P) -> P {
        P::new(self.x - p.x, self.y - p.y)
    }
}

#[derive(Clone, Debug)]
struct Machine {
    pos: P,
    v: P,
}

#[derive(Clone, Debug)]
struct State {
    me: Machine,
    you: Machine,
    turn: usize,
}

impl State {
    fn from_game_state(gs: rust_game_base::GameState) -> State {
        todo!();
    }

    fn mock() -> State {
        State {
            turn: 10,
            me: Machine {
                pos: P::new(50, 50),
                v: P::new(0, 0),
            },
            you: Machine {
                pos: P::new(-50, -50),
                v: P::new(0, 0),
            },
        }
    }

    fn finished(&self) -> bool {
        // TODO
        self.turn == 0
    }
}

// the higher the better
fn evaluate_state(s: &State) -> f64 {
    1.0 / (s.me.pos.dist(s.you.pos) + 1.0)
}

// get next states without actually running the action.
fn next_state(s: &State, a: &Action) -> State {
    // TODO

    if a.len() < 2 {}
    s.clone()
}

type Action = Vec<Command>;

#[derive(Debug, Clone)]
enum Command {
    Thrust(i8, i8), // (dx, dy)
    Bomb,
    Beam { x: isize, y: isize, power: usize },
}

fn possible_actions(s: &State) -> Vec<Action> {
    vec![]
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
