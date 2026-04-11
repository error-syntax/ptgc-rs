use anyhow::Result;
use inquire::{InquireError, Select, Text};

mod pokemon;

fn main() {
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

        println!("Searching for {}...", query.unwrap());
        break;
      },
      Ok("sets") => {
        let query = Text::new("What Set are you looking for?").prompt();

        if query.is_err() {
          eprintln!("Invalid entry...")
        }

        println!("Searching for {}...", query.unwrap());
        break;
      }
      Ok(choice) => println!("Unknown choice: {}", choice),
      Err(_) => println!("Invalid selection"),
    }
  }
}
