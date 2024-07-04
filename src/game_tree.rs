use self::GameTreeNode::{FirstMoves, GameOver, RandomEvent, SecondMoves};
use crate::game_tree::strategy::{FirstStrategy, NaiveStrategy, SecondStrategy};
use float_cmp::assert_approx_eq;
use rules::{GameRules, Move, Observation, State};
use std::hash::Hash;
use std::marker::PhantomData;

pub mod rules;
pub mod strategy;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum GameTreeNode<M: Move> {
    RandomEvent(Vec<(M, usize, f64)>),
    FirstMoves(Vec<(M, usize)>),
    SecondMoves(Vec<(M, usize)>),
    GameOver(f64),
}

impl<M: Move> GameTreeNode<M> {
    fn borrow_random_event_mut(&mut self) -> &mut Vec<(M, usize, f64)> {
        match self {
            RandomEvent(w) => w,
            _ => panic!(),
        }
    }

    fn borrow_player_moves_mut(&mut self) -> &mut Vec<(M, usize)> {
        match self {
            FirstMoves(w) | SecondMoves(w) => w,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameTree<M: Move, F: Observation<M>, S: Observation<M>, R: GameRules<M, F, S>> {
    rules: R,
    nodes: Vec<GameTreeNode<M>>,
    _phantom_f: PhantomData<F>,
    _phantom_s: PhantomData<S>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Play<M: Move, F: Observation<M>, S: Observation<M>> {
    moves: Vec<M>,
    first: Vec<F>,
    second: Vec<S>,
}

impl<M: Move, F: Observation<M>, S: Observation<M>> Default for Play<M, F, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Move, F: Observation<M>, S: Observation<M>> Play<M, F, S> {
    fn new() -> Self {
        Self {
            moves: vec![],
            first: vec![],
            second: vec![],
        }
    }

    fn push_move(&mut self, event: M) {
        self.moves.push(event);
        self.first.push(event.into());
        self.second.push(event.into());
    }

    fn pop_move(&mut self) -> Option<M> {
        self.first.pop();
        self.second.pop();
        self.moves.pop()
    }

    fn to_arbiter(&self) -> &[M] {
        &self.moves
    }

    fn to_first(&self) -> &[F] {
        &self.first
    }

    fn to_second(&self) -> &[S] {
        &self.second
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.moves.len()
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }
}

impl<M: Move, F: Observation<M>, S: Observation<M>, R: GameRules<M, F, S>> GameTree<M, F, S, R> {
    fn build(nodes: &mut Vec<GameTreeNode<M>>, path: &mut Play<M, F, S>, rules: &R) {
        let index = nodes.len();

        match rules.ask_arbiter(path.to_arbiter()) {
            State::RandomEvent => {
                let events = rules.random_event(path.to_arbiter());
                let n = events.len();

                assert_approx_eq!(f64, events.iter().map(|(_, prob)| prob).sum(), 1.);

                nodes.push(RandomEvent(
                    events
                        .into_iter()
                        .map(|(x, y)| (x, usize::MAX, y))
                        .collect(),
                ));

                for j in 0..n {
                    let u = nodes.len();
                    let event = &mut nodes[index].borrow_random_event_mut()[j];
                    event.1 = u;
                    path.push_move(event.0);
                    Self::build(nodes, path, rules);
                    path.pop_move();
                }
            }
            State::FirstToMove => {
                let events = rules.ask_first(path.to_first());
                let n = events.len();

                nodes.push(FirstMoves(
                    events.into_iter().map(|x| (x, usize::MAX)).collect(),
                ));

                for j in 0..n {
                    let u = nodes.len();
                    let event = &mut nodes[index].borrow_player_moves_mut()[j];
                    event.1 = u;
                    path.push_move(event.0);
                    Self::build(nodes, path, rules);
                    path.pop_move();
                }
            }
            State::SecondToMove => {
                let events = rules.ask_second(path.to_second());
                let n = events.len();

                nodes.push(SecondMoves(
                    events.into_iter().map(|x| (x, usize::MAX)).collect(),
                ));

                for j in 0..n {
                    let u = nodes.len();
                    let event = &mut nodes[index].borrow_player_moves_mut()[j];
                    event.1 = u;
                    path.push_move(event.0);
                    Self::build(nodes, path, rules);
                    path.pop_move();
                }
            }
            State::GameOver(value) => nodes.push(GameOver(value)),
        }
    }

    pub fn from_rules(rules: R) -> Self {
        let mut nodes = vec![];
        let mut path = Play::new();

        Self::build(&mut nodes, &mut path, &rules);

        assert!(path.is_empty());

        Self {
            nodes,
            rules,
            _phantom_f: PhantomData,
            _phantom_s: PhantomData,
        }
    }

    fn dfs(
        &self,
        f: &impl FirstStrategy<M, F>,
        s: &impl SecondStrategy<M, S>,
        path: &mut Play<M, F, S>,
        v: usize,
    ) -> f64 {
        match &self.nodes[v] {
            RandomEvent(row) => {
                let mut sum = 0.;

                for (m, u, p) in row {
                    path.push_move(*m);
                    sum += self.dfs(f, s, path, *u) * p;
                    path.pop_move();
                }

                sum
            }
            FirstMoves(row) => {
                let m = f.make_move::<S>(path.to_first(), &self.rules);

                let u = row
                    .iter()
                    .copied()
                    .find_map(|(a, b)| if a == m { Some(b) } else { None })
                    .unwrap();

                path.push_move(m);
                let ans = self.dfs(f, s, path, u);
                path.pop_move();
                ans
            }
            SecondMoves(row) => {
                let m = s.make_move::<F>(path.to_second(), &self.rules);

                let u = row
                    .iter()
                    .copied()
                    .find_map(|(a, b)| if a == m { Some(b) } else { None })
                    .unwrap();

                path.push_move(m);
                let ans = self.dfs(f, s, path, u);
                path.pop_move();
                ans
            }
            GameOver(x) => *x,
        }
    }

    pub fn simulate(&self, f: &impl FirstStrategy<M, F>, s: &impl SecondStrategy<M, S>) -> f64 {
        self.dfs(f, s, &mut Play::new(), 0)
    }

    pub fn strategy_matrix(
        &self,
        f: &[impl FirstStrategy<M, F>],
        s: &[impl SecondStrategy<M, S>],
    ) -> Vec<Vec<f64>> {
        f.iter()
            .map(|fs| s.iter().map(|ss| self.simulate(fs, ss)).collect())
            .collect()
    }

    pub fn to_matrix(&self) -> Vec<Vec<f64>> {
        self.strategy_matrix(
            &self.list_all_first_strategies(),
            &self.list_all_second_strategies(),
        )
    }

    fn check_first_strategy(
        &self,
        play: &mut Play<M, F, S>,
        strategy: &NaiveStrategy<M, F>,
        v: usize,
    ) -> Option<Vec<NaiveStrategy<M, F>>> {
        match &self.nodes[v] {
            FirstMoves(row) => {
                let turn = strategy.get(play.to_first());

                match turn {
                    None => Some(
                        row.iter()
                            .map(|(m, _)| {
                                let mut q = strategy.clone();
                                q.insert(Vec::from(play.to_first()), *m);
                                q
                            })
                            .collect(),
                    ),
                    Some(m) => {
                        play.push_move(*m);
                        let ans = self.check_first_strategy(
                            play,
                            strategy,
                            row.iter().find(|(rm, _)| rm == m).unwrap().1,
                        );
                        play.pop_move();
                        ans
                    }
                }
            }
            RandomEvent(row) => row
                .iter()
                .map(|(m, u, _)| {
                    play.push_move(*m);
                    let ans = self.check_first_strategy(play, strategy, *u);
                    play.pop_move();
                    ans
                })
                .find(|w| w.is_some())
                .flatten(),
            SecondMoves(row) => row
                .iter()
                .map(|(m, u)| {
                    play.push_move(*m);
                    let ans = self.check_first_strategy(play, strategy, *u);
                    play.pop_move();
                    ans
                })
                .find(|w| w.is_some())
                .flatten(),
            GameOver(_) => None,
        }
    }

    fn check_second_strategy(
        &self,
        play: &mut Play<M, F, S>,
        strategy: &NaiveStrategy<M, S>,
        v: usize,
    ) -> Option<Vec<NaiveStrategy<M, S>>> {
        match &self.nodes[v] {
            SecondMoves(row) => {
                let turn = strategy.get(play.to_second());

                match turn {
                    None => Some(
                        row.iter()
                            .map(|(m, _)| {
                                let mut q = strategy.clone();
                                q.insert(Vec::from(play.to_second()), *m);
                                q
                            })
                            .collect(),
                    ),
                    Some(m) => {
                        play.push_move(*m);
                        let ans = self.check_second_strategy(
                            play,
                            strategy,
                            row.iter().find(|(rm, _)| rm == m).unwrap().1,
                        );
                        play.pop_move();
                        ans
                    }
                }
            }
            RandomEvent(row) => row
                .iter()
                .map(|(m, u, _)| {
                    play.push_move(*m);
                    let ans = self.check_second_strategy(play, strategy, *u);
                    play.pop_move();
                    ans
                })
                .find(|w| w.is_some())
                .flatten(),
            FirstMoves(row) => row
                .iter()
                .map(|(m, u)| {
                    play.push_move(*m);
                    let ans = self.check_second_strategy(play, strategy, *u);
                    play.pop_move();
                    ans
                })
                .find(|w| w.is_some())
                .flatten(),
            GameOver(_) => None,
        }
    }

    pub fn list_all_first_strategies(&self) -> Vec<NaiveStrategy<M, F>> {
        let mut ans = Vec::new();
        let mut stack = vec![NaiveStrategy::new()];

        while let Some(s) = stack.pop() {
            match self.check_first_strategy(&mut Play::new(), &s, 0) {
                None => ans.push(s),
                Some(updated) => stack.extend_from_slice(&updated),
            }
        }

        ans
    }

    pub fn list_all_second_strategies(&self) -> Vec<NaiveStrategy<M, S>> {
        let mut ans = Vec::new();
        let mut stack = vec![NaiveStrategy::new()];

        while let Some(s) = stack.pop() {
            match self.check_second_strategy(&mut Play::new(), &s, 0) {
                None => ans.push(s),
                Some(updated) => stack.extend_from_slice(&updated),
            }
        }

        ans
    }
}
