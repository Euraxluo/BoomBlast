// use crate::cards::*;
// /// Module for the main game
// use crate::deck::*;
// use crate::player::*;

// use std::fmt;

// pub struct Game {
//     pub deck: Deck,
//     pub discard_pile: Vec<Box<dyn Card>>,
//     pub players: Vec<Player>,
//     pub current_player: Player,
//     pub turn_count: usize,
//     pub cards_to_draw: usize,
//     pub next_player_cards_to_draw: usize,
//     pub winner: Player,
// }

// impl Game {
//     pub fn new(players: &[&str]) -> Self {
//         let mut game = Game {
//             deck: Deck::new(players.len()),

//             discard_pile: Vec::new(),
//             players: Vec::new(),
//             current_player: Player::default(),
//             turn_count: 0,
//             cards_to_draw: 1,
//             next_player_cards_to_draw: 0,
//             winner: Player::default(),
//         };

//         for player_name in players {
//             let mut player = Player::new(player_name);
//             player.deal_hand((0..5).map(|_| game.deck.cards.pop().unwrap()).collect());
//             player.hand.push(Box::new(Defuse));
//             player.hand.sort_by_key(|card| card.id);
//             game.players.push(player);
//         }

//         game.deck.add_defuse_and_exploding_to_deck();
//         game
//     }

//     fn update_discard_pile(&mut self, played_card: Box<dyn Card>) {
//         if played_card.id() != 0 {
//             self.discard_pile.push(played_card);
//         }
//     }

//     fn execute_all_turns_for_a_player(&mut self) {
//         self.next_player_cards_to_draw = 1;
//         while self.cards_to_draw != 0 {
//             let mut end_turn = false;
//             while !end_turn {
//                 if let Some(played_card) = self.current_player.choose_card_to_play() {
//                     println!(
//                         "Player {} plays {}",
//                         self.current_player,
//                         played_card.to_string()
//                     );
//                     self.update_discard_pile(played_card.clone());
//                     played_card.action(self);
//                     if self.cards_to_draw == 0 {
//                         break;
//                     }
//                 } else {
//                     end_turn = true;
//                 }
//             }

//             if self.cards_to_draw == 0 {
//                 break;
//             }

//             let drawn_card = self.current_player.draw_to_end_turn(&mut self.deck);
//             println!(
//                 "Player {} draws {}",
//                 self.current_player,
//                 drawn_card.to_string()
//             );

//             if drawn_card.id() == 0 {
//                 let exploded = self.current_player.explode();
//                 if exploded {
//                     self.current_player.active = false;
//                     self.update_discard_pile(Box::new(drawn_card));
//                     println!("Bye bye player {}", self.current_player);
//                     self.cards_to_draw = 1;
//                 }
//             }

//             self.cards_to_draw -= 1;
//         }
//     }

//     pub fn play_game(&mut self) {
//         let mut end_of_game = false;
//         let mut remaining_players = vec![];
//         while !end_of_game {
//             let player_id = self.turn_count % self.players.len();
//             self.current_player = self.players[player_id].clone();
//             self.current_player.id = player_id;

//             if self.current_player.active {
//                 self.execute_all_turns_for_a_player();
//                 self.cards_to_draw = self.next_player_cards_to_draw;
//             }

//             remaining_players = self.players.iter().filter(|pl| pl.active).collect();
//             if remaining_players.len() == 1 {
//                 end_of_game = true;
//             }

//             self.turn_count += 1;
//         }

//         self.winner = remaining_players[0].clone();
//         println!("The winner is {}! Hooray yay wohoohoho!!!", self.winner);
//     }
// }

// impl fmt::Display for Game {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Game")
//     }
// }
