use crate::game_tree::GameTree;
use crate::matrix_game::{reverse_game, solve_game};
use games::{Guess, RockPaperScissors};

mod game_tree;
mod games;
mod matrix_game;

fn main() {
    let game = GameTree::from_rules(Guess::default()).to_matrix();

    dbg!(&game);

    dbg!(solve_game(&game));
    dbg!(solve_game(&reverse_game(&game)));

    let game = GameTree::from_rules(RockPaperScissors::default()).to_matrix();

    dbg!(&game);

    dbg!(solve_game(&game));
    dbg!(solve_game(&reverse_game(&game)));
}
