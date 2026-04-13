use anyhow::{Context, Result};
use dotenvy::var;
use reqwest;

use crate::models::{ApiList, Card};

pub async fn where_name(query: &str) -> Result<Vec<Card>> {
  let api_base_url = var("POKEMON_TCG_API_BASE_URL").expect("Pokémon API Base URL not set");

  let url = format!("{}/cards?q=name:\"{}*\"", api_base_url, urlencoding::encode(query));
  let resp = reqwest::get(&url)
    .await
    .context("network error")?
    .json::<ApiList<Card>>()
    .await
    .context("deserialize error");

  match resp {
      Ok(cards) => { return Ok(cards.data) },
      Err(err) => { panic!("{:?}", err)},
  };
}

pub async fn where_set_id(set_id: String) -> Result<Vec<Card>> {
  let api_base_url = var("POKEMON_TCG_API_BASE_URL").expect("Pokémon API Base URL not set");

  let url = format!("{}/cards?q=set.id:\"{}\"&orderBy=number", api_base_url, urlencoding::encode(&set_id));
  let resp = reqwest::get(&url)
    .await
    .context("Network Error")?
    .json::<ApiList<Card>>()
    .await
    .context("Deserialization Error");

  match resp {
    Ok(cards) => Ok(cards.data),
    Err(e) => panic!("{:?}", e),
  }
}
