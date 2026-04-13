use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
  pub id: String,
  pub name: String,
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
  pub set: SetStub,
  number: String,
  artist: Option<String>,
  rarity: Option<String>,
  flavor_text: Option<String>,
  national_pokedex_numbers: Option<Vec<u16>>,
  legalities: Legalities,
  pub images: CardImages,
  pub tcgplayer: Option<TcgPlayer>,
  pub cardmarket: Option<CardMarket>,
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
  pub name: String,
  series: String,
  printed_total: u16,
  total: u16,
  legalities: Legalities,
  ptcgo_code: Option<String>,
  release_date: String,
  updated_at: Option<String>,
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
  pub small: String,
  pub large: String,
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
  pub updated_at: Option<String>,
  pub prices: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMarket {
  url: String,
  pub updated_at: Option<String>,
  pub prices: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
  pub id: String,
  pub name: String,
  pub series: String,
  printed_total: u16,
  pub total: u16,
  legalities: Legalities,
  ptcgo_code: Option<String>,
  release_date: String,
  updated_at: Option<String>,
  images: SetImages,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiList<T> {
  pub data: Vec<T>,
  pub page: Option<u8>,
  pub page_size: Option<u8>,
  pub ount: Option<u32>,
  pub total_count: Option<u32>,
}
