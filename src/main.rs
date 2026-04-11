use anyhow::Result;
use inquire::{InquireError, Select, Text};

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
              for set in &sets {
                println!("{:?}", set);
                println!("=========================================");
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
