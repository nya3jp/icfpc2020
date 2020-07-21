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

// use rust_game_base::*;

// struct Searcher {
//     depth: usize,
//     evaluator: Box<dyn Evaluator>,
// }

// trait Evaluator {
//     // 最初に一回だけ呼ばれる．
//     // 対処できる盤面であれば，state をセットする．
//     // そうでなければ false を返す．
//     // 例：defender 専用 evaluator の場合，attacker だったら false を返す．
//     fn set_stage_data(&mut self, s: &StageData) -> bool;

//     fn get_param(&self) -> Param;

//     // 現在の state に対し，次におこす Action の候補を返す．
//     fn next_action_candidates(&mut self, current_state: &CurrentState) -> Vec<Vec<rust_game_base::Command>>;

//     // evaluate the score of the state.
//     // The bigger the better.
//     fn score(&mut self, state: &CurrentState) -> f64;
// }

// impl Searcher {
//     fn new(evaluator: Box<dyn Evaluator>) -> Searcher {
//         Searcher {
//             depth: 1,
//             evaluator,
//         }
//     }

//     fn start_game(es: Vec<Box<dyn Evaluator>>) {
//         let resp = rust_game_base::send_join_request().unwrap();
//         let stage_data = resp.stage_data.clone();

//         for e in es {
//             if e.set_stage_data(&stage_data) {
//                 Searcher::new(e).run(&stage_data);
//                 return;
//             }
//         }
//         panic!("no evaluator can handle {:?}", stage_data);
//     }

//     fn run(&mut self, stage_data: &StageData) {
//         rust_game_base::send_start_request(&rust::)
//     }
// }

// struct SimpleEval {
//     stage_data: StageData
// }

// impl Evaluator for SimpleEval {
//     fn set_stage_data(&mut self, s: &StageData) -> bool {
//         self.stage_data = s.clone();
//         s.is_attacker()
//     }
// }

// fn main() {
//     let eval = SimpleEval{};
//     let mut searcher = Searcher::new(Box::new(eval));

//     searcher.start_game();
// }

// #[cfg(test)]
// mod tests {
//     fn test() {
//         assert_eq!(1+1,2);
//     }
// }
