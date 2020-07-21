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

use anyhow::{anyhow, bail, Context, Result};
use rust_game_base::*;
use std::{
    cmp::{max, min},
    collections::VecDeque,
};

mod sa;

use rand::Rng;
use sa::*;

// 知見
// レーザーの威力はmin(初期設定のlaser_power, 発射時のパラメーター)
// heatは実際に発射された威力分上昇

struct Bot {
    stage: CurrentGameState,
    static_info: StageData,
    state: CurrentState,
    cmd_queue: VecDeque<Vec<Command>>,
}

// static VECT: &[Point] = &[
//     Point::new(-1, -1),
//     Point::new(-1, 0),
//     Point::new(-1, 1),
//     Point::new(0, -1),
//     Point::new(0, 1),
//     Point::new(1, -1),
//     Point::new(1, 0),
//     Point::new(1, 1),
//     Point::new(0, 0),
// ];

#[derive(Clone)]
struct Problem {
    static_info: StageData,
    state: CurrentState,
    me: Machine,
}

impl Problem {
    fn gen_rand_move(&self, rng: &mut impl rand::Rng) -> (i64, i64) {
        let lim = self.me.move_limit as i64;
        (rng.gen_range(-lim, lim + 1), rng.gen_range(-lim, lim + 1))
    }
}

impl Annealer for Problem {
    type State = Vec<(i64, i64)>;
    type Move = (usize, (i64, i64), (i64, i64));

    fn init_state(&self, rng: &mut impl rand::Rng) -> Self::State {
        let rest_step = min(10, self.static_info.total_turns - self.state.turn);
        let mut v = vec![(0, 0); rest_step];

        for r in v.iter_mut() {
            *r = self.gen_rand_move(rng);
        }

        v
    }

    fn eval(&self, state: &Self::State) -> f64 {
        let rest_step = self.static_info.total_turns - self.state.turn;

        let mut cur = self.me.position;
        let mut heat = self.me.heat;
        let mut velo = self.me.velocity;

        let mut ovh_pena = 0;
        let mut comp_step = 0;
        let mut use_acc = 0;
        let mut last_use = 0;

        for step in 0..rest_step {
            let (dx, dy) = if step < state.len() {
                state[step]
            } else {
                (0, 0)
            };

            velo -= Point::new(dx as _, dy as _);
            velo += get_gravity(&self.state, &cur);
            cur += velo;

            if !is_safe(&self.static_info, &cur) {
                break;
            }

            if dx != 0 || dy != 0 {
                heat += max(dx.abs(), dy.abs()) as usize * 8;
                use_acc += 1;
                last_use = step;
            }

            heat -= min(heat, self.me.params.cool_down_per_turn);

            if heat > self.me.heat_limit {
                let over = heat - self.me.heat_limit;
                ovh_pena += over;
                heat = self.me.heat_limit;
            }

            comp_step += 1;
        }

        // dbg!(comp_step, state.len(), rest_step, use_acc, ovh_pena);

        let score = if comp_step < rest_step {
            // 完走できず
            1000 + rest_step - comp_step
        } else {
            // アクセル使った回数にオーバーヒートペナを加味して
            // -> やっぱ最終利用にしたほうがよさそう
            last_use + ovh_pena * 10
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
            let (dx, dy) = self.gen_rand_move(rng);

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

fn is_safe(stage: &StageData, p: &Point) -> bool {
    if let Some(obs) = &stage.obstacle {
        if p.x.abs() <= obs.gravity_radius as isize && p.y.abs() <= obs.gravity_radius as isize {
            return false;
        }
        if p.x.abs() > obs.stage_half_size as isize || p.y.abs() > obs.stage_half_size as isize {
            return false;
        }
    }
    true
}

impl Bot {
    fn new() -> Result<Bot> {
        let resp = send_join_request()?;
        Ok(Bot {
            stage: resp.current_game_state,
            static_info: resp.stage_data,
            state: Default::default(),
            cmd_queue: VecDeque::new(),
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

        dbg!(self.static_info.initialize_param.heat_limit);

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

                // laser_power 極振りがよさそう？
                // cool_down_per_turnはもっといるだろ

                match &self.static_info.defender {
                    Some(ene) if ene.life >= 20 => {
                        // 超分裂タイプか？
                        // cool_down_per_turn をでかめにしたい

                        if param_rest >= 12
                            && param.cool_down_per_turn < 14
                            && param.cool_down_per_turn * 12 <= param.laser_power * 3
                        {
                            param.cool_down_per_turn += 1;
                            param_rest -= 12;
                            continue;
                        }

                        if param_rest >= 4
                            && param.laser_power
                                < min(96, self.static_info.initialize_param.heat_limit) as usize
                            && param.laser_power * 3 <= param.energy * 4
                        {
                            param.laser_power += 1;
                            param_rest -= 4;
                            continue;
                        }
                    }
                    _ => {
                        // 分裂しなさそう
                        // laser_power 極振りでいく
                        if param_rest >= 12
                            && param.cool_down_per_turn < 8
                            && param.cool_down_per_turn * 12 <= param.energy * 2
                            && param.cool_down_per_turn * 12 <= param.laser_power * 4
                        {
                            param.cool_down_per_turn += 1;
                            param_rest -= 12;
                            continue;
                        }

                        if param_rest >= 4
                            && param.laser_power
                                < min(96, self.static_info.initialize_param.heat_limit) as usize
                            && param.laser_power * 2 <= param.energy * 4
                        {
                            param.laser_power += 1;
                            param_rest -= 4;
                            continue;
                        }
                    }
                }
            } else {
                // ディフェンダーはパワーに振らない
                // スラスターを使い放題にするためにcool_down_per_turnを8にしたい
                // あとはライフとエネルギーを均等に
                // エネルギー多めの方がよさそう

                if param_rest >= 12
                    && param.cool_down_per_turn < 4
                    && param.cool_down_per_turn * 12 <= param.energy
                {
                    param.cool_down_per_turn += 1;
                    param_rest -= 12;
                    continue;
                }

                if param_rest >= 2 && param.life * 7 <= param.energy * 3 {
                    param.life += 1;
                    param_rest -= 2;
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

        let cmds = if self.get_me().role == Role::ATTACKER {
            self.attacker()
        } else {
            self.defender()
        };

        self.apply_response(send_command_request(&mut cmds.into_iter())?);
        Ok(())
    }

    fn attacker(&mut self) -> Vec<Command> {
        dbg!(self.static_info.self_role);
        // dbg!(&self.state);

        // コマンドキューが残ってるならそれを使う
        if let Some(cs) = self.cmd_queue.pop_front() {
            return cs;
        }

        let mut cmds = vec![];

        // * 楕円軌道に乗る
        // * 位置関係がいい感じならビームを打つ
        // * 分裂をどうするか

        // TODO: アタッカー：偶然自爆で殺せる位置にいたとき、自爆する
        // TODO: ディフェンダー：打たれそうになったら、移動して直撃を避けようとする

        // 周回軌道に乗るってるか？
        if self.state.turn + self.live_time(self.get_me(), &Point::new(0, 0))
            < self.static_info.total_turns && self.get_me().params.energy > 0
        {
            // 乗ってない
            // 周回軌道へ速やかに乗る
            eprintln!("Searching geocentric orbit...");

            let mut sol = annealing(
                &Problem {
                    static_info: self.static_info.clone(),
                    state: self.state.clone(),
                    me: self.get_me().clone(),
                },
                &AnnealingOptions::new(0.4, 1000.0, 0.1),
            );

            while !sol.is_empty() && sol[sol.len() - 1] == (0, 0) {
                sol.pop();
            }

            let mut first = true;

            for (dx, dy) in sol {
                let cs = vec![Command::Thrust(
                    self.get_me().machine_id,
                    Point::new(dx as _, dy as _),
                )];

                if first {
                    first = false;
                    cmds = cs;
                } else {
                    self.cmd_queue.push_back(cs);
                }
            }
        }

        if cmds.is_empty() {
            if self.get_me().role == Role::ATTACKER {
                // いい感じのポジショニングならビームを打つ

                let me = self.get_me();

                let heat_mergin = me.heat_limit as usize + me.params.cool_down_per_turn - me.heat;

                // 次の位置予測
                let next_me_pos =
                    me.position + me.velocity + get_gravity(&self.state, &me.position);

                let can_move = next_me_pos.x.abs() > self.grav_area() * 3 / 2
                    || next_me_pos.y.abs() > self.grav_area() * 3 / 2;

                let mut best_dmg = (0, 0, 0);
                let mut cand = None;

                for m in self.state.machines.iter() {
                    let m = m.0;
                    if m.role == self.get_me().role {
                        continue;
                    }

                    let ene = m;

                    let next_ene_pos =
                        ene.position + ene.velocity + get_gravity(&self.state, &ene.position);

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if !can_move && (dx, dy) != (0, 0) {
                                continue;
                            }

                            // ここにはいないだろう
                            if !is_safe(&self.static_info, &next_ene_pos) {
                                continue;
                            }

                            let v = next_me_pos - Point::new(dx, dy) - next_ene_pos;

                            // // スイートスポット以外は打たない
                            // let zure =
                            //     min(v.x.abs(), min(v.y.abs(), (v.x.abs() - v.y.abs()).abs()));
                            // if zure >= 9 {
                            //     continue;
                            // }

                            // ずれてるときは移動しながら打つのでヒートに余裕を見る
                            let max_beam_pow = min(
                                heat_mergin as isize - max(dx.abs(), dy.abs()) * 8,
                                me.params.laser_power as isize,
                            );

                            if max_beam_pow <= 0 {
                                continue;
                            }

                            let dmg =
                                rust_game_base::get_intensity(&v, max_beam_pow as usize) as isize;

                            let dmg = min(
                                dmg - (ene.heat_limit + ene.params.cool_down_per_turn - ene.heat)
                                    as isize,
                                (ene.params.energy
                                    + ene.params.laser_power
                                    + ene.params.cool_down_per_turn
                                    + ene.params.life) as isize,
                            );

                            // 与えるダメージが変わらない範囲で出力を下げる
                            let mut max_beam_pow = max_beam_pow;
                            while max_beam_pow > 1 {
                                max_beam_pow -= 1;
                                let t = rust_game_base::get_intensity(&v, max_beam_pow as usize)
                                    as isize;

                                let t = min(
                                    t - (ene.heat_limit + ene.params.cool_down_per_turn - ene.heat)
                                        as isize,
                                    (ene.params.energy
                                        + ene.params.laser_power
                                        + ene.params.cool_down_per_turn
                                        + ene.params.life)
                                        as isize,
                                );

                                if t != dmg {
                                    max_beam_pow += 1;
                                    break;
                                }
                            }

                            let dd = -(dx.abs() + dy.abs());

                            // let dist = -(v.x.abs() + v.y.abs());

                            // 同じダメージなら使うエネルギーが少ないところを打ちたい気がする
                            let dist = -(max_beam_pow + max(dx.abs(), dy.abs()) * 8);

                            if (dmg, dist, dd) <= best_dmg {
                                continue;
                            }

                            best_dmg = (dmg, dist, dd);
                            cand = Some((dx, dy, max_beam_pow, next_ene_pos));
                        }
                    }
                }

                if let Some((dx, dy, beam_pow, pos)) = cand {
                    if dx != 0 || dy != 0 {
                        cmds.push(Command::Thrust(me.machine_id, Point::new(dx, dy)));
                    }
                    eprintln!("Beam: expected dmg = {}", best_dmg.0);
                    cmds.push(Command::Beam(me.machine_id, pos, beam_pow as _));
                }
            }
        }

        cmds
    }

    fn defender(&mut self) -> Vec<Command> {
        dbg!(self.static_info.self_role);
        eprintln!("Current ships: {}", self.state.machines.len());
        // dbg!(&self.state);

        // コマンドキューが残ってるならそれを使う
        if let Some(cs) = self.cmd_queue.pop_front() {
            return cs;
        }

        let mut cmds = vec![];

        // なるべく早く周回軌道を見つける

        // 周回軌道に乗るってるか？
        if self.state.turn + self.live_time(self.get_leader(), &Point::new(0, 0))
            < self.static_info.total_turns
        {
            // 乗ってない
            // 周回軌道へ速やかに乗る
            eprintln!("Searching geocentric orbit...");

            // FIXME: DFSかIDにでもする
            let mut sol = annealing(
                &Problem {
                    static_info: self.static_info.clone(),
                    state: self.state.clone(),
                    me: self.get_leader().clone(),
                },
                &AnnealingOptions::new(0.4, 1000.0, 0.1),
            );

            while !sol.is_empty() && sol[sol.len() - 1] == (0, 0) {
                sol.pop();
            }

            let mut first = true;

            for (dx, dy) in sol {
                let cs = vec![Command::Thrust(
                    self.get_leader().machine_id,
                    Point::new(dx as _, dy as _),
                )];

                if first {
                    first = false;
                    cmds = cs;
                } else {
                    self.cmd_queue.push_back(cs);
                }
            }
        } else {
            // ライフが2以上あれば、分裂する
            // その後ランダムに移動する

            let np = self.next_pos(self.get_leader());

            if self.get_leader().params.energy >= 5
                && self.get_leader().params.life >= 2
                && (np.x.abs() > self.grav_area() * 3 / 2 || np.y.abs() > self.grav_area() * 3 / 2)
            {
                eprintln!("Splitting...");

                if self.leader_num() == 1 {
                    cmds.push(Command::Split(
                        self.get_leader().machine_id,
                        Param {
                            energy: self.get_leader().params.energy / 2,
                            laser_power: 0,
                            cool_down_per_turn: 0,
                            life: self.get_leader().params.life / 2 - 1,
                        },
                    ));
                } else {
                    cmds.push(Command::Split(
                        self.get_leader().machine_id,
                        Param {
                            energy: 0,
                            laser_power: 0,
                            cool_down_per_turn: 0,
                            life: 1,
                        },
                    ));
                }

                let (dx, dy) = 'outer: loop {
                    // let dx = rand::thread_rng().gen_range(-1, 2);
                    // let dy = rand::thread_rng().gen_range(-1, 2);
                    // if (dx, dy) != (0, 0) {
                    //     break (dx, dy);
                    // }

                    // 近傍で最後まで回れるところがあるならそこに行く。
                    // そうじゃなければ外側に向かうように

                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if self.state.turn
                                + self.live_time(self.get_leader(), &Point::new(dx, dy))
                                < self.static_info.total_turns
                            {
                                eprintln!("stable adj point found!");
                                break 'outer (dx, dy);
                            }
                        }
                    }

                    // fallback
                    break (
                        -(self.get_leader().position.x.signum() as isize),
                        -(self.get_leader().position.y.signum() as isize),
                    );
                };

                self.cmd_queue.push_back(vec![Command::Thrust(
                    self.get_leader().machine_id,
                    Point::new(dx, dy),
                )]);
            } else {
                // if we have enough bots, and the attacker is only one.
                // try to give some damage to the attacker.
                let machines = self.get_machines();
                if machines.len() > 12 && self.get_num_enemies() == 1 {
                    let enemy = self.get_some_enemy();

                    let next_ene_pos = self.next_pos(enemy);
                    let mut dmg = 0;
                    let mut remaining = 4;
                    'search: for d in 0..=3 {
                        for m in machines.iter() {
                            let next_pos = self.next_pos(m);
                            let diff = next_pos - next_ene_pos;
                            let distance = max(diff.x.abs(), diff.y.abs());
                            if d == distance {
                                dmg += 128 - 32 * d;
                                cmds.push(Command::Bomb(m.machine_id));
                                remaining -= 1;
                                if remaining == 0 {
                                    break 'search;
                                }
                            }
                        }
                    }
                }
            }
        }

        cmds
    }

    fn next_pos(&self, m: &Machine) -> Point {
        m.position + m.velocity + get_gravity(&self.state, &m.position)
    }

    fn grav_area(&self) -> isize {
        if let Some(obs) = &self.static_info.obstacle {
            obs.gravity_radius as isize
        } else {
            0
        }
    }

    fn live_time(&self, r: &Machine, acc: &Point) -> usize {
        let step_limit = self.static_info.total_turns - self.state.turn;

        let mut ret = 0;
        let mut v = r.velocity - *acc;
        let mut cur = r.position;

        while ret < step_limit {
            v += get_gravity(&self.state, &cur);
            cur += v;

            if !is_safe(&self.static_info, &cur) {
                break;
            }

            ret += 1;
        }

        dbg!(ret);

        ret
    }

    // 自分の中で、一番でかいやつを見つける
    fn get_me(&self) -> &Machine {
        &self
            .state
            .machines
            .iter()
            .filter(|r| r.0.role == self.static_info.self_role)
            .max_by_key(|r| r.0.params.life)
            .expect("Cannot find me")
            .0
    }

    fn get_leader(&self) -> &Machine {
        self.state
            .machines
            .iter()
            .filter(|r| r.0.role == self.static_info.self_role)
            .filter(|r| r.0.params.life > 1 && r.0.params.energy >= 5)
            .min_by_key(|r| r.0.machine_id)
            .map_or_else(|| self.get_me(), |r| &r.0)
    }

    fn get_machines(&self) -> Vec<&Machine> {
        self.state
            .machines
            .iter()
            .filter(|r| r.0.role == self.static_info.self_role)
            .filter(|r| {
                r.0.params.energy == 0
                    && r.0.params.laser_power == 0
                    && r.0.params.cool_down_per_turn == 0
                    && r.0.params.life == 1
            })
            .map(|r| &r.0)
            .collect()
    }

    fn leader_num(&self) -> usize {
        self.state
            .machines
            .iter()
            .filter(|r| r.0.role == self.static_info.self_role)
            .filter(|r| r.0.params.life > 1)
            .count()
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

    fn get_num_enemies(&self) -> usize {
        self.state
            .machines
            .iter()
            .filter(|r| r.0.role != self.static_info.self_role)
            .count()
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
