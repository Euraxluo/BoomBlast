// use rand::prelude::*;
// use crate::{cards::*, constants::*};

// pub struct Deck {
//     pub num_of_players: usize,
//     pub cards: Vec<Box<dyn Card>>,
// }

// impl Deck {
//     pub fn new(num_of_players: usize) -> Self {
//         let mut deck = Deck {
//             num_of_players,
//             cards: Vec::new(),
//         };
//         deck.initialise_deck();
//         deck
//     }

    
//     fn initialise_deck(&mut self) {
//         self.cards.extend((0..NUM_OF_ATTACK_CARDS_IN_BASE_GAME).map(|_| Box::new(Attack) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_SHUFFLE_CARDS_IN_BASE_GAME).map(|_| Box::new(Shuffle) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_SEE_FUTURE_CARDS_IN_BASE_GAME).map(|_| Box::new(SeeTheFuture) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_NOPE_CARDS_IN_BASE_GAME).map(|_| Box::new(Nope) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_SKIP_CARDS_IN_BASE_GAME).map(|_| Box::new(Skip) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_FAVOR_CARDS_IN_BASE_GAME).map(|_| Box::new(Favor) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_TACOCAT_CARDS_IN_BASE_GAME).map(|_| Box::new(Tacocat) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_CATERMELON_CARDS_IN_BASE_GAME).map(|_| Box::new(Catermelon) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_HAIRY_POTATO_CAT_CARDS_IN_BASE_GAME).map(|_| Box::new(HairyPotatoCat) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_BEARD_CAT_CARDS_IN_BASE_GAME).map(|_| Box::new(BeardCat) as Box<dyn Card>));
//         self.cards.extend((0..NUM_OF_RAINBOW_RALPHING_CAT_CARDS_IN_BASE_GAME).map(|_| Box::new(RainbowCat) as Box<dyn Card>));
//         self.shuffle_deck();
//     }

//     pub fn add_defuse_and_exploding_to_deck(&mut self) {
//         self.cards.extend(
//             (0..(NUM_OF_DEFUSE_CARDS_IN_BASE_GAME - self.num_of_players))
//                 .map(|_| Box::new(Defuse) as Box<dyn Card>),
//         );
//         self.cards.extend(
//             (0..(self.num_of_players - 1))
//                 .map(|_| Box::new(ExplodingKitten) as Box<dyn Card>),
//         );
//         self.shuffle_deck();
//     }

//     pub fn shuffle_deck(&mut self) {
//         let mut rng = rand::thread_rng();
//         self.cards.shuffle(&mut rng);
//     }
// }
