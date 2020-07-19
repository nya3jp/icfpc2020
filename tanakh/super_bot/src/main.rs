use anyhow::{anyhow, bail, Context, Result};
use rust_game_base::*;

struct Bot {
    stage: CurrentGameState,
    static_info: StageData,
    state: CurrentState,
}

// main (args)
// {
//     // parse command line arguments
//     serverUrl = args[0]
//     playerKey = args[1]

//     // make valid JOIN request using the provided playerKey
//     joinRequest = makeJoinRequest(playerKey)

//     // send it to aliens and get the GameResponse
//     gameResponse = send(serverUrl, joinRequest)

//     // make valid START request using the provided playerKey and gameResponse returned from JOIN
//     startRequest = makeStartRequest(playerKey, gameResponse)

//     // send it to aliens and get the updated GameResponse
//     gameResponse = send(serverUrl, startRequest)

//     while (true) // todo: you MAY detect somehow that game is finished using gameResponse
//     {
//         // make valid COMMANDS request using the provided playerKey and gameResponse returned from START or previous COMMANDS
//         commandsRequest = makeCommandsRequest(playerKey, gameResponse)

//         // send it to aliens and get the updated GameResponse
//         gameResponse = send(serverUrl, commandsRequest)
//     }
// }

impl Bot {
    fn new() -> Result<Bot> {
        let resp = send_join_request()?;
        Ok(Bot {
            stage: resp.current_game_state,
            static_info: resp.stage_data,
            state: Default::default(),
        })
    }

    fn start(&mut self) -> Result<()> {
        let param = Param {
            energy: 0,
            laser_power: 0,
            life: 0,
            cool_down_per_turn: 0,
        };

        self.apply_response(send_start_request(&param)?);
        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        let cmds = vec![];

        self.apply_response(send_command_request(&mut cmds.into_iter())?);
        Ok(())
    }

    fn apply_response(&mut self, resp: Response) {
        self.stage = resp.current_game_state;
        assert_eq!(self.static_info, resp.stage_data);
        self.state = resp.current_state.unwrap();
    }
}

fn main() -> Result<()> {
    let mut bot = Bot::new()?;
    bot.start()?;
    while bot.stage != CurrentGameState::END {
        bot.step()?;
    }
    Ok(())
}
