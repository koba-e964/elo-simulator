use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub participants: Vec<Participant>,
    pub kind: KindConfig,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub decided: Vec<Decided>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub queries: Vec<Query>,
}

impl GameConfig {
    pub fn decided_matrix(&self) -> Vec<Vec<i32>> {
        let n = self.participants.len();
        let mut result = vec![vec![0; n]; n];
        for d in &self.decided {
            let winner_index = self
                .participants
                .iter()
                .position(|participant| participant.name == d.winner);
            let loser_index = self
                .participants
                .iter()
                .position(|participant| participant.name == d.loser);
            if let (Some(winner_index), Some(loser_index)) = (winner_index, loser_index) {
                result[winner_index][loser_index] = 1;
                result[loser_index][winner_index] = -1;
            }
        }
        result
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Participant {
    pub name: String,
    pub rating: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub is_absent: bool,
}

impl Debug for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.rating)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KindConfig {
    Elimination,
    RoundRobin,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Decided {
    pub winner: String,
    pub loser: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub name: String,
    #[serde(flatten)]
    pub query: QueryInner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueryInner {
    Within(usize),
    _NonExhaustive,
}

fn is_false(&a: &bool) -> bool {
    !a
}

pub type Probability = f64;
pub fn win_prob(rating_me: f64, rating_other: f64) -> Probability {
    // 400 higher, 10 times likely to win.
    let odds = (10.0_f64).powf((rating_me - rating_other) / 400.0);
    odds / (1.0 + odds)
}
