use elo_simulator::entity::*;
use elo_simulator::{brute, sim};
use toml::from_slice;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    eprintln!("filename = {}", filename);
    let dat = std::fs::read(filename)?;
    let config: GameConfig = from_slice(&dat).unwrap();
    println!("config = {:?}", config);
    let probs = sim::sim(config.clone());
    let probs_exact = brute::brute(config.clone());
    println!(
        "|{}\t|{}\t|{}\t|{}\t|{}\t|",
        "参加者",
        config.queries[0].name,
        config.queries[0].name.clone() + "(厳密)",
        config.queries[1].name,
        config.queries[1].name.clone() + "(厳密)",
    );
    println!("|---|---|---|---|---|");
    for i in 0..config.participants.len() {
        if config.participants[i].is_absent {
            continue;
        }
        println!(
            "|{:?}\t|{}\t|{}\t|{}\t|{}\t|",
            config.participants[i],
            display_prob(probs[0][i]),
            display_prob(probs_exact[0][i]),
            display_prob(probs[1][i]),
            display_prob(probs_exact[1][i]),
        );
    }
    Ok(())
}

fn display_prob(p: Probability) -> String {
    if p < 1.0e-13 {
        return format!("{:.4e}", p);
    }
    if p >= 0.001 {
        return format!("{:.4}%", 100.0 * p);
    }
    if p >= 1.0e-6 {
        return format!("{:.4} ppm", 1.0e6 * p);
    }
    format!("{:.4} ppb", 1.0e9 * p)
}
