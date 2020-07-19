use anyhow::{anyhow, bail, Context, Result};
use rust_game_base::*;
use std::cmp::{max, min};

mod sa;

use sa::*;

// 知見
// レーザーの威力はmin(初期設定のlaser_power, 発射時のパラメーター)
// heatは実際に発射された威力分上昇

const MAX_STEP: usize = 256;

#[derive(Clone)]
struct Bot {
    stage: CurrentGameState,
    static_info: StageData,
    state: CurrentState,
}

static VECT: &[Point] = &[
    Point::new(-1, -1),
    Point::new(-1, 0),
    Point::new(-1, 1),
    Point::new(0, -1),
    Point::new(0, 1),
    Point::new(1, -1),
    Point::new(1, 0),
    Point::new(1, 1),
    Point::new(0, 0),
];

#[derive(Clone)]
struct Problem {
    bot: Bot,
}

impl Annealer for Problem {
    type State = Vec<(i64, i64)>;
    type Move = (usize, (i64, i64), (i64, i64));

    fn init_state(&self, rng: &mut impl rand::Rng) -> Self::State {
        let rest_step = self.bot.static_info.total_turns - self.bot.state.turn;
        let mut v = vec![(0, 0); rest_step];

        for r in v.iter_mut() {
            r.0 = rng.gen_range(-1, 2);
            r.1 = rng.gen_range(-1, 2);
        }

        v
    }

    fn eval(&self, state: &Self::State) -> f64 {
        let me = self.bot.get_me();

        let mut cur = me.position;
        let mut heat = me.heat;
        let mut velo = me.velocity;

        let mut ovh_pena = 0;
        let mut comp_step = 0;
        let mut use_acc = 0;

        for &(dx, dy) in state.iter() {
            velo -= Point::new(dx as _, dy as _);
            velo += get_gravity(&self.bot.state, &cur);
            cur += velo;

            if !self.bot.is_safe(&cur) {
                break;
            }

            if dx != 0 || dy != 0 {
                heat += 8;
                use_acc += 1;
            }

            heat -= min(heat, me.params.cool_down_per_turn);

            if heat > 64 {
                let over = heat - 64;
                ovh_pena += over;
                heat = 64;
            }

            comp_step += 1;
        }

        // dbg!(comp_step, state.len(), use_acc, ovh_pena);

        let score = if comp_step < state.len() {
            // 完走できず
            1000 + state.len() - comp_step
        } else {
            // アクセル使った回数にオーバーヒートペナを加味して
            use_acc + ovh_pena * 10
        };

        score as f64
    }

    fn neighbour(
        &self,
        state: &Self::State,
        rng: &mut impl rand::Rng,
        _progress: f64,
    ) -> Self::Move {
        loop {
            let ix = rng.gen_range(0, state.len());
            let dx = rng.gen_range(-1, 2);
            let dy = rng.gen_range(-1, 2);

            if (dx, dy) == state[ix] {
                continue;
            }

            break (ix, (dx, dy), state[ix]);
        }
    }

    fn apply(&self, state: &mut Self::State, mov: &Self::Move) {
        state[mov.0] = mov.1;
    }

    fn unapply(&self, state: &mut Self::State, mov: &Self::Move) {
        state[mov.0] = mov.2;
    }
}

impl Bot {
    fn new() -> Result<Bot> {
        let resp = send_join_request()?;
        Ok(Bot {
            stage: resp.current_game_state,
            static_info: resp.stage_data,
            state: Default::default(),
        })
    }

    fn apply_response(&mut self, resp: Response) {
        self.stage = resp.current_game_state;
        assert_eq!(self.static_info, resp.stage_data);
        self.state = resp.current_state.unwrap();
    }

    fn start(&mut self) -> Result<()> {
        assert_eq!(self.stage, CurrentGameState::START);
        dbg!(&self.static_info);

        // FIXME: ???
        let mut param_rest = self.static_info.initialize_param.total_cost;

        let mut param = Param {
            energy: 0,
            laser_power: 0,
            life: 0,
            cool_down_per_turn: 0,
        };

        // パラメーター割り振り
        // attackerの時はレーザーに多めに、
        // defenderの時はレーザーに割り振らないようにするといい？

        while param_rest > 0 {
            if param.life == 0 {
                param.life += 1;
                param_rest -= 2;
                continue;
            }

            if self.static_info.self_role == Role::ATTACKER {
                // アタッカーはまんべんなく振る
                // cool_down_per_turn はこっちも8で十分か？
                // TODO: もっといいパラメーターあるかも

                if param_rest >= 12
                    && param.cool_down_per_turn < 8
                    && param.cool_down_per_turn * 12 <= param.energy
                    && param.cool_down_per_turn * 12 <= param.laser_power * 4
                {
                    param.cool_down_per_turn += 1;
                    param_rest -= 12;
                    continue;
                }

                if param_rest >= 4
                    && param.laser_power < 64
                    && param.laser_power * 2 <= param.energy
                    && (param.cool_down_per_turn >= 8
                        || param.laser_power * 2 <= param.cool_down_per_turn * 12)
                {
                    param.laser_power += 1;
                    param_rest -= 4;
                    continue;
                }
            } else {
                // ディフェンダーはパワーに振らない
                // cool_down_per_turnは8あれば十分

                if param_rest >= 12
                    && param.cool_down_per_turn < 8
                    && param.cool_down_per_turn * 12 <= param.energy
                {
                    param.cool_down_per_turn += 1;
                    param_rest -= 12;
                    continue;
                }
            }

            // ライフに振る必要ある？

            // if param_rest >= 2
            //     && param.life * 2 <= param.energy
            //     && param.life * 2 <= param.laser_power * 4
            //     && param.life * 2 <= param.cool_down_per_turn * 12
            // {
            //     param.life += 1;
            //     param_rest -= 2;
            //     continue;
            // }

            param.energy += 1;
            param_rest -= 1;
        }

        dbg!(&param);

        assert!(
            param.energy + param.laser_power * 4 + param.life * 2 + param.cool_down_per_turn * 12
                <= self.static_info.initialize_param.total_cost as usize
        );

        self.apply_response(send_start_request(&param)?);
        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        assert_eq!(self.stage, CurrentGameState::PLAYING);
        let cmds = self.think();
        self.apply_response(send_command_request(&mut cmds.into_iter())?);
        Ok(())
    }

    fn think(&mut self) -> Vec<Command> {
        dbg!(self.static_info.self_role);
        dbg!(&self.state);

        let mut cmds = vec![];

        // * 楕円軌道に乗る
        // * 位置関係がいい感じならビームを打つ
        // * 分裂をどうするか

        // TODO: アタッカー：偶然自爆で殺せる位置にいたとき、自爆する
        // TODO: ディフェンダー：打たれそうになったら、移動して直撃を避けようとする

        let sol = annealing(
            &Problem { bot: self.clone() },
            &AnnealingOptions::new(0.2, 1000.0, 0.1),
        );

        if !sol.is_empty() {
            let dx = sol[0].0;
            let dy = sol[0].1;

            if dx != 0 || dy != 0 {
                cmds.push(Command::Thrust(
                    self.get_me().machine_id as _,
                    Point::new(dx as _, dy as _),
                ));
            }
        }

        if cmds.is_empty() {
            if self.get_me().role == Role::ATTACKER {
                // いい感じのポジショニングならビームを打つ

                let ene = self.get_some_enemy();
                let me = self.get_me();

                let heat_mergin = 64 - me.heat + me.params.cool_down_per_turn;

                // 次の位置予測
                let next_ene_pos =
                    ene.position + ene.velocity + get_gravity(&self.state, &ene.position);

                let next_me_pos =
                    me.position + me.velocity + get_gravity(&self.state, &me.position);

                let mut best_dmg = (0, 0);
                let mut cand = None;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        // ここにはいないだろう
                        if !self.is_safe(&next_ene_pos) {
                            continue;
                        }

                        let v = next_me_pos - Point::new(dx, dy) - next_ene_pos;

                        // スイートスポット以外は打たない
                        if !(v.x == 0 || v.y == 0 || v.x.abs() == v.y.abs()) {
                            continue;
                        }

                        // ずれてるときは移動しながら打つのでヒートに余裕を見る
                        let max_beam_pow = min(
                            heat_mergin as isize - if dx == 0 && dy == 0 { 0 } else { 8 },
                            me.params.laser_power as isize,
                        );

                        let decay = max(v.x.abs(), v.y.abs());
                        let dmg = max_beam_pow * 3 - decay;

                        let dd = -(dx.abs() + dy.abs());

                        if (dmg, dd) <= best_dmg {
                            continue;
                        }

                        best_dmg = (dmg, dd);
                        cand = Some((dx, dy, max_beam_pow));
                    }
                }

                if let Some((dx, dy, beam_pow)) = cand {
                    if dx != 0 || dy != 0 {
                        cmds.push(Command::Thrust(me.machine_id, Point::new(dx, dy)));
                    }
                    cmds.push(Command::Beam(me.machine_id, next_ene_pos, beam_pow as _));
                }
            }
        }

        cmds
    }

    // fn live_time(&self, r: &Machine, acc: &Point) -> usize {
    //     let step_limit = MAX_STEP - self.state.turn;

    //     let mut ret = 0;
    //     let mut v = r.velocity - *acc;
    //     let mut cur = r.position;

    //     while ret < step_limit {
    //         v += get_gravity(&self.state, &cur);
    //         cur += v;

    //         if !self.is_safe(&cur) {
    //             break;
    //         }

    //         ret += 1;
    //     }

    //     dbg!(ret);

    //     ret
    // }

    fn is_safe(&self, p: &Point) -> bool {
        if let Some(obs) = &self.static_info.obstacle {
            if p.x.abs() <= obs.gravity_radius as isize && p.y.abs() <= obs.gravity_radius as isize
            {
                return false;
            }

            if p.x.abs() > obs.stage_half_size as isize || p.y.abs() > obs.stage_half_size as isize
            {
                return false;
            }
        }
        true
    }

    fn get_me(&self) -> &Machine {
        for m in self.state.machines.iter() {
            let m = &m.0;
            if m.role == self.static_info.self_role {
                return m;
            }
        }
        panic!("Cannot find me")
    }

    fn get_some_enemy(&self) -> &Machine {
        for m in self.state.machines.iter() {
            let m = &m.0;
            if m.role != self.static_info.self_role {
                return m;
            }
        }
        panic!("Cannot find enemy")
    }
}

// Returns the gravity.
pub fn get_gravity(state: &CurrentState, pos: &Point) -> Point {
    if state.obstacle.is_none() {
        return Point { x: 0, y: 0 };
    }

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

pub fn clamp<T: Ord>(input: T, min_v: T, max_v: T) -> T {
    max(min_v, min(max_v, input))
}

fn main() -> Result<()> {
    let mut bot = Bot::new()?;
    bot.start()?;
    while bot.stage != CurrentGameState::END {
        bot.step()?;
    }
    Ok(())
}
