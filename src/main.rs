use inquire::{InquireError, Select};

mod commands;
mod models;
mod pokemon;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();

  loop {
    let options = vec!["cards", "sets"];
    println!("What would you like to do?");

    match Select::new("Search By:", options).prompt() {
      Ok("cards") => {
        commands::cards::search().await;
      }
      Ok("sets") => {
        commands::sets::search().await;
      }
      Ok(choice) => println!("Unknown choice: {}", choice),
      Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => break,
      Err(_) => println!("Invalid selection"),
    }
  }
}
