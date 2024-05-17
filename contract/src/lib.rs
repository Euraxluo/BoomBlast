pub mod game;
pub mod player;
pub mod test;
pub mod contract;

mod tests_game {

    use super::*;
    #[test]
    fn test() {
        println!("hello world");
    }
    // use crate::game::Game;

    // #[test]
    // fn test_game_init_default() {
    //     let testgame = Game::new_default();
    //     assert_eq!(testgame.players.len(), 2);
    //     assert_eq!(testgame.players[0].name, "Player 1");
    //     assert_eq!(testgame.players[1].name, "Player 2");
    //     assert_eq!(testgame.players[0].hand.len(), 6);
    //     assert_eq!(testgame.players[1].hand.len(), 6);
    //     assert_eq!(testgame.deck.cards.len(), 41);
    // }

    // #[test]
    // fn test_game_init_three_players() {
    //     let testgame = Game::new(&vec!["Vasos", "Mixas", "Kostas"]);
    //     assert_eq!(testgame.players.len(), 3);
    //     assert_eq!(testgame.players[0].name, "Vasos");
    //     assert_eq!(testgame.players[1].name, "Mixas");
    //     assert_eq!(testgame.players[2].name, "Kostas");
    //     assert_eq!(testgame.players[0].hand.len(), 6);
    //     assert_eq!(testgame.players[1].hand.len(), 6);
    //     assert_eq!(testgame.players[2].hand.len(), 6);
    //     assert_eq!(testgame.deck.cards.len(), 36);
    // }

    // #[test]
    // fn test_play() {
    //     let mut game = Game::new(&vec!["Alice", "Bob", "Charlie"]);
    //     game.play_game();
    // }
}
