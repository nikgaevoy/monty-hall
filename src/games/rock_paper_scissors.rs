use crate::game_tree::rules::{GameRules, State};
use self::Player::{First, Second};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RockPaperScissors {}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Player {
    First = 0,
    Second = 1,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PlayerGesture {
    Rock(Player),
    Paper(Player),
    Scissors(Player),
}

impl PlayerGesture {
    pub fn into_player(self) -> Player {
        match self {
            PlayerGesture::Rock(x) => x,
            PlayerGesture::Paper(x) => x,
            PlayerGesture::Scissors(x) => x,
        }
    }

    pub fn to_gesture(self) -> u8 {
        match self {
            PlayerGesture::Rock(_) => 0,
            PlayerGesture::Paper(_) => 1,
            PlayerGesture::Scissors(_) => 2,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Intent<const PLAYER: u8> {
    #[default]
    Unknown,
    Rock,
    Paper,
    Scissors,
}

impl<const PLAYER: u8> From<PlayerGesture> for Intent<{ PLAYER }> {
    fn from(value: PlayerGesture) -> Self {
        if value.into_player() as u8 == PLAYER {
            match value {
                PlayerGesture::Rock(_) => Self::Rock,
                PlayerGesture::Paper(_) => Self::Paper,
                PlayerGesture::Scissors(_) => Self::Scissors,
            }
        } else {
            Self::Unknown
        }
    }
}

fn compare_gestures(a: PlayerGesture, b: PlayerGesture) -> f64 {
    if a.to_gesture() == b.to_gesture() {
        0.
    } else if a.to_gesture() == (b.to_gesture() + 1) % 3 {
        1.
    } else {
        -1.
    }
}

impl GameRules<PlayerGesture, Intent<0>, Intent<1>> for RockPaperScissors {
    fn ask_arbiter(&self, moves: &[PlayerGesture]) -> State {
        match moves.len() {
            0 => State::FirstToMove,
            1 => State::SecondToMove,
            2 => State::GameOver(compare_gestures(moves[0], moves[1])),
            _ => unreachable!(),
        }
    }

    fn ask_first(&self, _moves: &[Intent<0>]) -> Vec<PlayerGesture> {
        vec![
            PlayerGesture::Rock(First),
            PlayerGesture::Paper(First),
            PlayerGesture::Scissors(First),
        ]
    }

    fn ask_second(&self, _moves: &[Intent<1>]) -> Vec<PlayerGesture> {
        vec![
            PlayerGesture::Rock(Second),
            PlayerGesture::Paper(Second),
            PlayerGesture::Scissors(Second),
        ]
    }

    fn random_event(&self, _moves: &[PlayerGesture]) -> Vec<(PlayerGesture, f64)> {
        unreachable!()
    }
}
