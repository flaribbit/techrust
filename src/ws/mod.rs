pub mod response;

use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct GetAction {
    action: i32,
}

#[derive(Deserialize)]
pub struct MatchStart {
    action: i32,
    errno: i32,
    seed: i32,
}

#[derive(Deserialize)]
pub struct PlayerConfig {
    data: String,
}

#[derive(Debug)]
pub enum Action {
    GlobalOnlineCount,
    MatchEnd,
    MatchReady,
    MatchStart { seed: i32 },
    PlayerConfig(String),
    // ...
}

#[derive(thiserror::Error, Debug)]
pub enum ParseActionError {
    #[error("Error parsing json: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Unknown action: {0}")]
    UnknownAction(i32),
}

impl std::str::FromStr for Action {
    type Err = ParseActionError;
    fn from_str(message: &str) -> Result<Self, Self::Err> {
        let action_id = serde_json::from_str::<GetAction>(message)?.action;
        match action_id {
            1000 => Ok(Action::GlobalOnlineCount),
            1100 => Ok(Action::MatchEnd),
            1101 => Ok(Action::MatchReady),
            1102 => {
                let o: MatchStart = serde_json::from_str(message)?;
                Ok(Action::MatchStart { seed: o.seed })
            }
            1103 => {
                let o: PlayerConfig = serde_json::from_str(message)?;
                Ok(Action::PlayerConfig(o.data))
            }
            _ => Err(ParseActionError::UnknownAction(action_id)),
        }
    }
}

#[test]
fn test2() -> Result<(), Box<dyn std::error::Error>> {
    let a: Action = r#"{"action": 1102, "errno": 0, "seed": 123}"#.parse()?;
    println!("{:?}", a);
    println!(
        "This should error: {:?}",
        r#"{"action": 1103,"#.parse::<Action>()
    );
    println!(
        "This should be unknown action: {:?}",
        r#"{"action": 114514}"#.parse::<Action>()
    );
    Ok(())
}

// not working
// impl<'de> serde::Deserialize<'de> for Action {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         let get_action = GetAction::deserialize(d)?;
//         match get_action.action {
//             1000 => Ok(Action::GlobalOnlineCount),
//             1100 => Ok(Action::MatchEnd),
//             1101 => Ok(Action::MatchReady),
//             1102 => {
//                 let match_start = MatchStart::deserialize(d)?;
//                 Ok(Action::MatchStart {
//                     seed: match_start.seed,
//                 })
//             }
//             1103 => {
//                 let player_config = PlayerConfig::deserialize(d)?;
//                 Ok(Action::PlayerConfig(player_config.data))
//             }
//             _ => Err(serde::de::Error::custom("unknown action")),
//         }
//     }
// }

#[test]
fn test() {
    println!("{}", response::global_online_count(114))
}
