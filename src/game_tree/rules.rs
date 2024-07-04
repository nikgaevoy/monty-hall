use std::hash::Hash;

pub trait Move: Copy + Eq + Hash {}

impl<T: Copy + Eq + Hash> Move for T {}

pub trait Observation<M>: Copy + Eq + Hash + From<M> {}

impl<M: Move, T: Copy + Eq + Hash + From<M>> Observation<M> for T {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum State {
    RandomEvent,
    FirstToMove,
    SecondToMove,
    GameOver(f64),
}

pub trait GameRules<M: Move, F: Observation<M>, S: Observation<M>> {
    fn ask_arbiter(&self, moves: &[M]) -> State;
    fn ask_first(&self, moves: &[F]) -> Vec<M>;
    fn ask_second(&self, moves: &[S]) -> Vec<M>;
    fn random_event(&self, moves: &[M]) -> Vec<(M, f64)>;
}
