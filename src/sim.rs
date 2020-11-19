use rand::{thread_rng, Rng as _};

use crate::entity::{GameConfig, KindConfig};

pub fn sim(config: GameConfig) -> Vec<Vec<Probability>> {
    // TODO: now only elimination is supported
    assert_eq!(config.kind, KindConfig::Elimination);
    let n = config.participants.len();
    assert!(n.is_power_of_two());

    // Run the simulation.
    const NUMBER_OF_SIMS: u64 = if cfg!(debug_assertions) {
        10_000
    } else {
        1_000_000
    };

    let mut wins = vec![0u64; n];
    let ratings: Vec<f64> = config.participants.iter().map(|x| x.rating).collect();

    let mut rng = thread_rng();

    for _ in 0..NUMBER_OF_SIMS {
        let mut rem: Vec<usize> = (0..n).collect();
        while rem.len() > 1 {
            let mut next = vec![];
            for i in 0..rem.len() / 2 {
                let a = ratings[rem[2 * i]];
                let b = ratings[rem[2 * i + 1]];
                let prob_a_win = win_prob(a, b);
                let a_win = rng.gen_bool(prob_a_win);
                if a_win {
                    next.push(rem[2 * i]);
                } else {
                    next.push(rem[2 * i + 1]);
                }
            }
            rem = next;
        }
        wins[rem[0]] += 1;
    }
    let mut probs = vec![0.0; n];
    for i in 0..n {
        if config.participants[i].is_absent {
            continue;
        }
        probs[i] = wins[i] as f64 / NUMBER_OF_SIMS as f64;
    }
    vec![probs]
}

pub type Probability = f64;

fn win_prob(rating_me: f64, rating_other: f64) -> Probability {
    // 400 higher, 10 times likely to win.
    let odds = (10.0_f64).powf((rating_me - rating_other) / 400.0);
    odds / (1.0 + odds)
}
