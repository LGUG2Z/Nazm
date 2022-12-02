use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser)]
pub struct Diff {
    /// Path to the nazm.toml file
    pub file: PathBuf,
}

#[derive(Parser)]
pub struct Apply {
    /// Path to the nazm.toml file
    pub file: PathBuf,
}

#[derive(Parser)]
pub enum SubCommand {
    /// Export current settings to terminal
    Export,
    /// Show the diff between the current system state and the given Nazm TOML file
    Diff(Diff),
    /// Apply changes from the given Nazm TOML file to the system
    Apply(Apply),
}
