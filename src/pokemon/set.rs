use anyhow::{Context, Result};
use dotenvy::var;
use reqwest;

use crate::models::{ApiList, Set};

pub async fn all() -> Result<Vec<Set>> {
  println!("Fetching all Sets...");

  let api_base_url = var("POKEMON_TCG_API_BASE_URL").expect("Pokémon API Base URL not set");

  let url = format!("{}/sets?orderBy=\"-releaseDate\"", api_base_url);
  let resp = reqwest::get(&url)
    .await
    .context("Network Error")?
    .json::<ApiList<Set>>()
    .await
    .context("Deserialize Error");

  match resp {
    Ok(sets) => { return Ok(sets.data) },
    Err(err) => { panic!("{:?}", err) },
  }
}
