use std::env;
use urlencoding;
use anyhow::{Context, Result};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
  id: String,
  name: String,
  supertype: String,
  subtypes: Option<Vec<String>>,
  level: Option<String>,
  hp: Option<String>,
  types: Option<Vec<String>>,
  evolves_from: Option<String>,
  evolves_to: Option<Vec<String>>,
  rules: Option<Vec<String>>,
  abilities: Option<Vec<Ability>>,
  attacks: Option<Vec<Attack>>,
  weaknesses: Option<Vec<Weakness>>,
  resistances: Option<Vec<Resistance>>,
  retreat_cost: Option<Vec<String>>,
  converted_retreat_cost: Option<u8>,
  set: SetStub,
  number: String,
  artist: Option<String>,
  rarity: Option<String>,
  flavor_text: Option<String>,
  national_pokedex_numbers: Option<Vec<u16>>,
  legalities: Legalities,
  images: CardImages,
  tcgplayer: Option<TcgPlayer>,
  cardmarket: Option<CardMarket>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
  name: String,
  text: String,
  #[serde(rename = "type")]
  ability_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
  name: String,
  cost: Vec<String>,
  converted_energy_cost: u8,
  damage: String,
  text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weakness {
  #[serde(rename = "type")]
  weakness_type: String,
  value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resistance {
  #[serde(rename = "type")]
  resistance_type: String,
  value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStub {
  id: String,
  name: String,
  series: String,
  printed_total: u16,
  total: u16,
  legalities: Legalities,
  ptcgo_code: Option<String>,
  release_date: String,
  updated_at: String,
  images: SetImages,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetImages {
  symbol: String,
  logo: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardImages {
  small: String,
  large: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legalities {
  unlimited: Option<String>,
  standard: Option<String>,
  expanded: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TcgPlayer {
  url: String,
  updated_at: String,
  prices: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMarket {
  url: String,
  updated_at: String,
  prices: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
  id: String,
  name: String,
  series: String,
  printed_total: u16,
  total: u16,
  legalities: Legalities,
  ptcgo_code: Option<String>,
  release_date: String,
  updated_at: String,
  images: SetImages,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiList<T> {
  data: Vec<T>,
  page: Option<u8>,
  page_size: Option<u8>,
  count: Option<u32>,
  total_count: Option<u32>,
}

async fn fetch_cards(query: &str) -> Result<Vec<Card>> {
  let api_base_url = env::var("POKEMON_TCG_API_BASE_URL").expect("Pokémon API Base URL not set");

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
