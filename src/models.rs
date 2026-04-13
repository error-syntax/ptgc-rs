use serde::{Deserialize, Serialize};

pub trait ShowPrices {
  fn show_prices(&self);
}

fn to_title_case(s: &str) -> String {
  let chars: Vec<char> = s.chars().collect();
  let mut result = String::new();

  for (i, &ch) in chars.iter().enumerate() {
    if i == 0 {
      result.extend(ch.to_uppercase());
    } else {
      let prev = chars[i - 1];
      if (ch.is_uppercase() && prev.is_lowercase())
        || (ch.is_ascii_digit() && !prev.is_ascii_digit())
        || (ch.is_alphabetic() && prev.is_ascii_digit())
      {
        result.push(' ');
      }
      result.push(ch);
    }
  }
  result
}

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

impl ShowPrices for TcgPlayer {
  fn show_prices(&self) {
    println!("Prices:");
    if let Some(prices) = &self.prices {
      if let Some(variants) = prices.as_object() {
        for (variant, price_data) in variants {
          println!("  {}:", to_title_case(variant));
          if let Some(fields) = price_data.as_object() {
            for (key, val) in fields {
              println!("    {}: {}", to_title_case(key), val);
            }
          }
        }
      }
    }
    println!("Updated: {}", self.updated_at.as_deref().unwrap_or("N/A"));
  }
}

impl ShowPrices for CardMarket {
  fn show_prices(&self) {
    println!("Prices:");
    if let Some(prices) = &self.prices {
      if let Some(fields) = prices.as_object() {
        for (key, val) in fields {
          println!("  {}: {}", to_title_case(key), val);
        }
      }
    }
    println!("Updated: {}", self.updated_at.as_deref().unwrap_or("N/A"));
  }
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
