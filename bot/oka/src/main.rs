#![allow(non_snake_case, unused, non_upper_case_globals)]

extern crate rust_game_base;
#[macro_use]
extern crate itertools;

/*
仕様があまり定まっていないので，generic な感じの AI を書く．

- 攻撃側であることを仮定．
- 分裂はしないことにする (自機はつねにひとつ．)．
- 相手は次の段階では動かないことを仮定．

- 上下左右にだけ動く．速度は 1 か 0 のみ．
*/

static mut DEV: bool = false;

fn is_dev() -> bool {
    unsafe { DEV }
}

fn main() {
    if std::env::args().nth(1) == Some("dev".to_owned()) {
        unsafe { DEV = true }
    }
    Proxy::join_game();
    let state = Proxy::start_game(&StartParam {});
    run(state);
}

struct Proxy;

impl Proxy {
    fn join_game() {
        if is_dev() {
            return;
        }
        // TODO
        // rust_game_base::send_join_request();
    }
    fn start_game(p: &StartParam) -> State {
        if is_dev() {
            return State::dummy();
        }

        // TODO
        let state = rust_game_base::send_start_request(4, 4, 4, 4);
        State::from_response(state)
    }

    fn do_action(a: &Action) -> State {
        // TODO

        State::dummy()
    }
}

struct StartParam {}

fn run(mut state: State) {
    loop {
        let mut best = (-10000.0, vec![], None);
        for a in possible_actions(&state) {
            let ns = next_state(&state, &a);
            let val = evaluate_state(&ns);
            if best.0 < val {
                best = (val, a, Some(ns));
            }
        }
        let got_state = Proxy::do_action(&best.1);

        // TODO: log if state != got_state.

        state = best.2.unwrap();
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

impl std::ops::Add for P {
    type Output = P;
    fn add(self, p: Self) -> Self::Output {
        P::new(self.x + p.x, self.y + p.y)
    }
}

impl std::ops::Sub for P {
    type Output = P;
    fn sub(self, p: Self) -> Self::Output {
        P::new(self.x - p.x, self.y - p.y)
    }
}

impl P {
    fn new(x: isize, y: isize) -> P {
        P { x, y }
    }
    fn dist(&self, p: P) -> f64 {
        self.dist2(p).sqrt()
    }
    fn dist2(&self, p: P) -> f64 {
        (*self - p).norm2()
    }
    fn norm2(&self) -> f64 {
        let (x, y) = (self.x, self.y);
        (x * x + y * y) as _
    }
    fn norm(&self) -> f64 {
        self.norm2().sqrt()
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
    fn from_response(gs: rust_game_base::Response) -> State {
        todo!();
    }

    fn dummy() -> State {
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
    const small: f64 = 1.0 / 1000.0;
    1.0 / (s.me.pos.dist(s.you.pos) + 1.0) - small * s.me.v.norm()
}

// get next states without actually running the action.
fn next_state(s: &State, a: &Action) -> State {
    // TODO

    let mut s = s.clone();
    s.turn -= 1;

    for c in a.iter() {
        match c {
            Command::Thrust(a) => {
                s.me.v = s.me.v + *a;
                s.me.pos = s.me.pos + s.me.v;
            }
            _ => unimplemented!(),
        }
    }
    s
}

type Action = Vec<Command>;

#[derive(Debug, Clone)]
enum Command {
    // 自機の速度を変える。指定したそのターンから即時効果を発揮する。
    // e.g.: Pos(0 . 0), V(1 . 0) の時に Thruster を A(1 . 0) 吹くと、次のターンでは Pos(2 . 0), V(2 . 0)
    Thrust(P), // (dx, dy)
    Bomb,
    Beam { dir: P, power: usize },
}

fn possible_actions(s: &State) -> Vec<Action> {
    let mut res = vec![];

    res.push(vec![]); // do nothing
    for (dx, dy) in iproduct!(-1..=1, -1..=1) {
        if dx == 0 && dy == 0 {
            continue;
        }
        res.push(vec![Command::Thrust(P::new(dx, dy))]);
    }
    res
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
