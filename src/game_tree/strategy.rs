use std::collections::HashMap;
use crate::game_tree::rules::{GameRules, Move, Observation};

pub trait FirstStrategy<M: Move, F: Observation<M>> {
    fn make_move<S: Observation<M>>(&self, play: &[F], rules: &impl GameRules<M, F, S>) -> M;
}

pub trait SecondStrategy<M: Move, S: Observation<M>> {
    fn make_move<F: Observation<M>>(&self, play: &[S], rules: &impl GameRules<M, F, S>) -> M;
}

pub type NaiveStrategy<M, F> = HashMap<Vec<F>, M>;

impl<M: Move, F: Observation<M>> FirstStrategy<M, F> for NaiveStrategy<M, F> {
    fn make_move<S: Observation<M>>(&self, play: &[F], _rules: &impl GameRules<M, F, S>) -> M {
        match self.get(play) {
            None => {
                unreachable!()
            }
            Some(m) => *m,
        }
    }
}

impl<M: Move, S: Observation<M>> SecondStrategy<M, S> for NaiveStrategy<M, S> {
    fn make_move<F: Observation<M>>(&self, play: &[S], _rules: &impl GameRules<M, F, S>) -> M {
        match self.get(play) {
            None => {
                unreachable!()
            }
            Some(m) => *m,
        }
    }
}
