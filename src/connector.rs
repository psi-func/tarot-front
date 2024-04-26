use reqwest::Client;
use serde;

use crate::config::{GET_EXPLANATION, GET_TAROT, SERVER_ADDRESS};

pub enum ReqType {
    TarotCards,
    TarotExplanation,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GetTarotCards {
    count: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TarotCards {
    pub cards: Vec<u16>,
}

pub async fn post_get_cards() -> Result<TarotCards, reqwest::Error> {
    let mut req = String::from(SERVER_ADDRESS);
    req.push_str(GET_TAROT);

    let client = Client::new();

    let body = GetTarotCards { count: 5 };

    match client
        .post(req.as_str())
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
    {
        Ok(resp) => {
            let data: TarotCards = resp.json().await?;
            Ok(data)
        }
        Err(err) => Err(err),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GetTarotExplanation {
    cards: Vec<u16>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TarotExplanation {
    exp: String,
}

pub async fn post_get_explanation(cards: Vec<u16>) -> Result<TarotCards, reqwest::Error> {
    let mut req = String::from(SERVER_ADDRESS);
    req.push_str(GET_EXPLANATION);

    let client = Client::new();

    let body = GetTarotExplanation { cards: cards };

    match client
        .post(req.as_str())
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
    {
        Ok(resp) => {
            let data: TarotCards = resp.json().await?;
            Ok(data)
        }
        Err(err) => Err(err),
    }
}