#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::redundant_pub_crate)]

use clap::Parser;
use nazm::Config;
use nazm::Opts;
use nazm::SubCommand;
use similar_asserts::SimpleDiff;

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Export => println!("{}", toml::to_string_pretty(&Config::export()?)?),
        SubCommand::Diff(args) => {
            let current = Config::export()?.to_string();
            let desired = Config::from_path(args.file)?.to_string();
            if current.eq(&desired) {
                println!("No differences found!");
                return Ok(());
            }

            let diff = SimpleDiff::from_str(&current, &desired, "system", "nazm.toml");
            println!("{}", diff);
        }
        SubCommand::Apply(args) => {
            let current = Config::export()?;
            let desired = Config::from_path(args.file)?;

            if current.eq(&desired) {
                println!("No changes required!");
                return Ok(());
            }

            let previous_state = current.apply()?.to_string();
            let new_state = desired.apply()?.to_string();

            let diff =
                SimpleDiff::from_str(&previous_state, &new_state, "previous state", "new state");

            println!("{}", diff);
        }
    }

    Ok(())
}
