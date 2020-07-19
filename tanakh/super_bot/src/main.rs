use anyhow::{anyhow, bail, Context, Result};
use rust_game_base::*;
use std::cmp::{max, min};

struct Bot {
    stage: CurrentGameState,
    static_info: StageData,
    state: CurrentState,
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
        let param_rest = self.static_info._2.0;

        let param = Param {
            energy: 128,
            laser_power: 10,
            life: 10,
            cool_down_per_turn: 64,
        };

        assert!(
            param.energy + param.laser_power + param.life + param.cool_down_per_turn
                <= param_rest as usize
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

        for m in self.state.machines.iter() {
            let m = m.0;
            if m.role != self.static_info.self_role {
                continue;
            }

            if m.velocity.x.abs() + m.velocity.y.abs() > 0 {
                let ax = clamp(m.velocity.x, -2, 2);
                let ay = clamp(m.velocity.y, -2, 2);

                dbg!(ax, ay);

                cmds.push(Command::Thrust(m.machine_id as _, Point::new(ax, ay)));
            }
        }

        cmds
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
