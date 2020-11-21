use rand::{thread_rng, Rng as _};

use crate::entity::{win_prob, GameConfig, KindConfig, Probability, QueryInner};

pub fn sim(config: GameConfig) -> Vec<Vec<Probability>> {
    // TODO: now only elimination is supported
    assert_eq!(config.kind, KindConfig::Elimination);
    let n = config.participants.len();
    assert!(n.is_power_of_two());
    let m = config.queries.len();

    let decided = config.decided_matrix();

    // Run the simulation.
    const NUMBER_OF_SIMS: u64 = if cfg!(debug_assertions) {
        10_000
    } else {
        1_000_000
    };

    let mut counts = vec![vec![0u64; n]; m];
    let ratings: Vec<f64> = config.participants.iter().map(|x| x.rating).collect();

    let mut rng = thread_rng();

    for _ in 0..NUMBER_OF_SIMS {
        let mut rem: Vec<usize> = (0..n).collect();
        while rem.len() > 1 {
            let mut next = vec![];
            for i in 0..rem.len() / 2 {
                let a = ratings[rem[2 * i]];
                let b = ratings[rem[2 * i + 1]];
                let mut prob_a_win = win_prob(a, b);
                match decided[rem[2 * i]][rem[2 * i + 1]] {
                    1 => prob_a_win = 1.0,
                    -1 => prob_a_win = 0.0,
                    _ => {}
                }
                let a_win = rng.gen_bool(prob_a_win);
                if a_win {
                    next.push(rem[2 * i]);
                } else {
                    next.push(rem[2 * i + 1]);
                }
            }
            rem = next;
            for (index, q) in config.queries.iter().enumerate() {
                if let &QueryInner::Within(number) = &q.query {
                    if rem.len() == number {
                        for i in 0..number {
                            counts[index][rem[i]] += 1;
                        }
                    }
                }
            }
        }
    }
    let mut probs = vec![vec![0.0; n]; m];
    for j in 0..m {
        for i in 0..n {
            if config.participants[i].is_absent {
                continue;
            }
            probs[j][i] = counts[j][i] as f64 / NUMBER_OF_SIMS as f64;
        }
    }
    probs
}
