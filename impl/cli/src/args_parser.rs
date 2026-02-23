use clap::{Parser, Subcommand};

use crate::tools;

#[derive(Parser)]
#[command(bin_name = "bit", version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

/// Subcommands
#[derive(Subcommand)]
pub enum SubCommand {
    Compile {
        filepath: String,

        #[arg(long, value_enum)]
        platform: Option<tools::Platform>
        // ...
    }
}

pub fn parse_args() -> Cli {
    Cli::parse()
}