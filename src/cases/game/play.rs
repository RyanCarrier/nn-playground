use crate::traits::generic_game_case::GenericGameCase;

pub fn play_game<I: Copy, GameCase: GenericGameCase<I>>(game: GameCase) {
    println!("play_game {:?}", game.title());
}
