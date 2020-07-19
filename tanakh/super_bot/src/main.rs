use anyhow::Result;
use rust_game_base::*;

struct Bot {}

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
        let resp = send_join_request();
        dbg!(resp);
        let resp = send_start_request(0, 0, 0, 0)?;
        Ok(Bot {})
    }
}

fn main() {
    let bot = Bot::new();
}
