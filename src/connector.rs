use reqwest::;
use serde_json::Value;

use crate::config::{GET_EXPLANATION, GET_TAROT, SERVER_ADDRESS};
use crate::error::Error;

pub enum RegType {
    TarotReadings,
    TarotExplanation,
}

pub fn get(rt: RegType) -> Result<serde_json::Value, Error> {
    let req = String::from(SERVER_ADDRESS);
    let path = match rt {
        RegType::TarotReadings => GET_TAROT,
        RegType::TarotExplanation => GET_EXPLANATION,
    };
    req.push_str(path);

    let client = Client::new();

    match client.get(req.as_str()) {
        Ok(resp) => {
            let data: serde_json::Value = resp.json().unwrap();
            Ok(data)
        }
        Err(err) => Err(Error::connection_error("sorry".into())),
    }
}
