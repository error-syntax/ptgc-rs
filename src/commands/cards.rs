use inquire::Text;

use crate::pokemon;

pub async fn search() {
  let query = Text::new("What Card are you looking for?").prompt();

  if query.is_err() {
    eprintln!("Invalid entry...");
    return;
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
    Err(e) => eprintln!("Error fetching cards: {}", e),
  }
}
