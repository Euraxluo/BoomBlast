// use std::fmt;
// use std::io;

// use crate::cards::*;
// use crate::game::*;

// #[derive(Debug, Clone)]
// pub struct Player {
//     pub name: String,
//     pub id: usize,
//     pub hand: Vec<Box<dyn Card>>,
//     pub active: bool,
// }

// impl Player {
//     pub fn new(name: &str) -> Self {
//         Player {
//             name: name.to_string(),
//             id: 0,
//             hand: Vec::new(),
//             active: true,
//         }
//     }

//     pub fn deal_hand(&mut self, hand: Vec<Box<dyn Card>>) {
//         self.hand = hand;
//     }

//     pub fn choose_card_to_play(&mut self) -> Option<Box<dyn Card>> {
//         // TODO: Add check to ensure valid card_id is selected

//         if self.name == "Mixas" {
//             println!("{:?} {:?}", self.name, self.hand);
//             let mut input = String::new();
//             io::stdin()
//                 .read_line(&mut input)
//                 .expect("Failed to read line");
//             let card_id: usize = input.trim().parse().expect("Invalid input");
//             if card_id == 0 {
//                 return None;
//             }
//             return Some(self.hand.remove(card_id - 1));
//         }

//         if rand::random::<f64>() < 0.3 {
//             return Some(self.hand.remove(rand::random::<usize>() % self.hand.len()));
//         }
//         None
//     }

//     pub fn draw_to_end_turn(&mut self, game: &mut Game) -> Box<dyn Card> {
//         let drawn_card = game.deck.cards.remove(0);
//         if drawn_card.id() != 0 {
//             self.hand.push(drawn_card.clone());
//         }
//         drawn_card
//     }

//     pub fn explode(&mut self) -> bool {
//         // TODO: Implement explosion
//         true
//     }
// }

// impl fmt::Display for Player {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.name)
//     }
// }
