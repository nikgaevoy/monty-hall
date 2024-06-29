use crate::game::{solve_game, reverse_game};

mod game;

fn main() {
    let game = vec![vec![0., 1., -1.],
                    vec![-1., 1., 1.],
                    vec![1., -1., 0.]];

    dbg!(solve_game(&game));
    dbg!(solve_game(&reverse_game(&game)));
}
