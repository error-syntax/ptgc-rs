use std::collections::HashMap;

use anyhow::Result;
use inquire::{InquireError, Select, Text};

use crate::models::Card;

mod models;
mod pokemon;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();

  loop {
    let options = vec!["cards", "sets"];
    println!("What would you like to do?");

    let ans: Result<&str, InquireError> = Select::new("Search By:", options).prompt();

    match ans {
      Ok("cards") => {
        let query = Text::new("What Card are you looking for?").prompt();

        if query.is_err() {
          eprintln!("Invalid entry...")
        }

        let query_string = query.unwrap();

        println!("Searching for {}...", &query_string);

        match pokemon::card::where_name(&query_string).await {
          Ok(cards) => {
            if cards.is_empty() {
              println!("No cards found.");
            } else {
              for card in &cards {
                println!("{} - {}", card.set.name, card.name);
                println!("Images: {:?}", card.images);
                println!("--------------------------------");
              }
            }
          },
          Err(e) => eprintln!("Error fetching cards: {}", e)
        }

        break;
      },
      Ok("sets") => {
        match pokemon::set::all().await {
          Ok(sets) => {
            if sets.is_empty() {
              println!("No sets found.");
            } else {
              let mut sets_map: HashMap<String, String> = HashMap::new();

              for set in &sets {
                sets_map
                  .entry(String::from(&set.id))
                  .or_insert(String::from(&set.name));
              }

              let selected_card_set = Select::new(
                "Which Set are you interested in?",
                sets_map.clone().into_values().collect()
              )
              .prompt();

              match selected_card_set {
                Ok(choice) => {
                  let set = sets_map.iter().find(|el| {
                    el.1.to_owned() == choice
                  });

                  match set {
                    Some(found) => {
                      println!("Fetching cards for Set: {}", found.1);
                      let set_cards = pokemon::card::where_set_id(found.0.to_owned()).await;

                      match set_cards {
                         Ok(cards) => {
                           let mut card_options: Vec<Card> = vec![];

                           for card in cards {
                             card_options.push(card);
                           }

                           println!("Here are all the cards: {:?}",card_options);
                         },
                        Err(_) => eprintln!("Something happened listing the Cards."),
                      }
                    },
                    None => eprintln!("Failed to fetch cards in Set"),
                  }

                },
                Err(_) => eprintln!("Failed to make a selection"),
              }
            }
          },
          Err(e) => eprintln!("Error fetching sets: {}", e),
        }
        break;
      }
      Ok(choice) => println!("Unknown choice: {}", choice),
      Err(_) => println!("Invalid selection"),
    }
  }
}
