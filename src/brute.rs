use crate::entity::{win_prob, GameConfig, Probability, QueryInner};

pub fn brute(config: GameConfig) -> Vec<Vec<Probability>> {
    let n = config.participants.len();
    let ratings: Vec<f64> = config.participants.iter().map(|x| x.rating).collect();
    let m = config.queries.len();
    assert!(n <= 32);
    assert!(n.is_power_of_two());

    let decided = config.decided_matrix();

    let mut probs = vec![vec![0.0; n]; m];

    // Try all patterns in the elimination
    for bits in 0..1 << (n - 1) {
        let mut pos = 0;
        let mut rem: Vec<usize> = (0..n).collect();
        let mut overall = 1.0;
        let mut added = vec![];
        while rem.len() > 1 {
            let mut next = vec![];
            for i in 0..rem.len() / 2 {
                let mut prob_a_win = win_prob(ratings[rem[2 * i]], ratings[rem[2 * i + 1]]);
                match decided[rem[2 * i]][rem[2 * i + 1]] {
                    1 => prob_a_win = 1.0,
                    -1 => prob_a_win = 0.0,
                    _ => {}
                }
                let (prob, win) = if (bits & 1 << pos) != 0 {
                    (prob_a_win, rem[2 * i])
                } else {
                    (1.0 - prob_a_win, rem[2 * i + 1])
                };
                overall *= prob;
                next.push(win);
                pos += 1;
            }
            rem = next;
            for (index, q) in config.queries.iter().enumerate() {
                if let &QueryInner::Within(number) = &q.query {
                    if rem.len() == number {
                        for i in 0..number {
                            added.push((index, rem[i]));
                        }
                    }
                }
            }
        }
        for (index, participant_index) in added {
            probs[index][participant_index] += overall;
        }
    }
    probs
}
