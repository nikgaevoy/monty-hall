use crate::game_tree::rules::State::{FirstToMove, GameOver, RandomEvent};
use crate::game_tree::rules::{GameRules, State};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Guess {}

impl GameRules<u8, u8, u8> for Guess {
    fn ask_arbiter(&self, moves: &[u8]) -> State {
        match moves.len() {
            0 => FirstToMove,
            1 => RandomEvent,
            2 => GameOver(if moves[0] == moves[1] { 1. } else { -1. }),
            _ => unreachable!(),
        }
    }

    fn ask_first(&self, _moves: &[u8]) -> Vec<u8> {
        vec![0, 1, 2]
    }

    fn ask_second(&self, _moves: &[u8]) -> Vec<u8> {
        unreachable!()
    }

    fn random_event(&self, _moves: &[u8]) -> Vec<(u8, f64)> {
        vec![(0, 0.25), (2, 0.75)]
    }
}
