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
    let m = config.queries.len();
    let mut column_names = Vec::with_capacity(2 * m);
    let mut columns = Vec::with_capacity(2 * m);
    for i in 0..m {
        column_names.push(config.queries[i].name.clone());
        column_names.push(config.queries[i].name.clone() + "(厳密)");
        columns.push(probs[i].clone());
        columns.push(probs_exact[i].clone());
    }
    display_table(&config.participants, &column_names, &columns);

    Ok(())
}

fn display_table(
    participants: &[Participant],
    column_names: &[String],
    columns: &[Vec<Probability>],
) {
    print!("|参加者\t|");
    for j in 0..columns.len() {
        print!("{}\t|", column_names[j]);
    }
    println!();
    print!("|---|");
    for _ in 0..columns.len() {
        print!("---|");
    }
    println!();
    for i in 0..participants.len() {
        if participants[i].is_absent {
            continue;
        }
        print!("|{:?}\t|", participants[i]);
        for j in 0..columns.len() {
            print!("{}\t|", display_prob(columns[j][i]));
        }
        println!();
    }
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
