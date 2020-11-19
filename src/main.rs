use elo_simulator::entity::*;
use toml::from_slice;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    eprintln!("filename = {}", filename);
    let dat = std::fs::read(filename)?;
    let config: GameConfig = from_slice(&dat).unwrap();
    println!("config = {:?}", config);
    Ok(())
}
