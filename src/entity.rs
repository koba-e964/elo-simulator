use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub participants: Vec<Participant>,
    pub kind: KindConfig,
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

fn is_false(&a: &bool) -> bool {
    !a
}
