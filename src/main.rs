use std::fs;
use toml;
use anyhow::anyhow;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}



fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    let toml = toml::from_str::<toml::Table>(fs::read_to_string("/home/dev/mangata-node/Cargo.toml")?.as_ref())?;
    let table: &toml::Value = toml.get("patch").ok_or(anyhow!("error"))?;
    if let toml::Value::Table(tab) = table {
        for k in tab.keys(){
            println!("{:?}", k);
        }
    }
    Ok(())
}
