use inquire::Select;

use crate::models::{Card, ShowPrices};
use crate::{images, pokemon};

pub async fn search() {
  let sets = match pokemon::set::all().await {
    Ok(sets) => sets,
    Err(e) => {
      eprintln!("Error fetching sets: {}", e);
      return;
    }
  };

  if sets.is_empty() {
    println!("No sets found.");
    return;
  }

  let selected = Select::new(
    "Which Set are you interested in?",
    sets.iter().map(|set| set.name.clone()).collect(),
  )
  .prompt();

  match selected {
    Ok(choice) => {
      let Some(found) = sets.iter().find(|el| el.name == choice) else {
        eprintln!("Failed to fetch cards in Set");
        return;
      };

      println!("Fetching cards for Set: {}", found.name);
      match pokemon::card::where_set_id(found.id.to_owned()).await {
        Ok(cards) => show_cards(cards).await,
        Err(_) => eprintln!("Something happened listing the Cards."),
      }
    }
    Err(_) => eprintln!("Failed to make a selection"),
  }
}

async fn show_cards(cards: Vec<Card>) {
  let names: Vec<String> = cards.iter().map(|c| c.name.clone()).collect();

  match Select::new("Choose a card:", names).prompt() {
    Ok(card_name) => match cards.iter().find(|card| card.name == card_name) {
      Some(card) => show_card_details(card).await,
      None => eprintln!("Unable to find Card"),
    },
    Err(_) => eprintln!("Failed to make a selection"),
  }
}

async fn show_card_details(card: &Card) {
  println!("Card: {}", card.name);

  if let Some(tcg) = &card.tcgplayer {
    println!("\nTCGPlayer");
    tcg.show_prices();
  }

  if let Some(cm) = &card.cardmarket {
    println!("\nCardMarket");
    cm.show_prices();
  }
}
