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
        "|{}\t|{}\t|{}\t|{}\t|",
        "参加者", "優勝確率", "優勝確率(厳密)", "本戦出場確率"
    );
    println!("|---|---|---|---|");
    for i in 0..config.participants.len() {
        if config.participants[i].is_absent {
            continue;
        }
        println!(
            "|{:?}\t|{}\t|{}\t|{}\t|",
            config.participants[i],
            display_prob(probs[0][i]),
            display_prob(probs_exact[0][i]),
            display_prob(probs[1][i]),
        );
    }
    Ok(())
}

fn display_prob(p: Probability) -> String {
    if p <= 1.0e-15 {
        return "0".to_owned();
    }
    if p >= 0.001 {
        return format!("{:.4}%", 100.0 * p);
    }
    if p >= 1.0e-6 {
        return format!("{:.4} ppm", 1.0e6 * p);
    }
    return (1.0e9 * p).to_string() + " ppb";
}
