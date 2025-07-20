use std::{fmt::Display, path::PathBuf};

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(flatten)]
    pub generate: GenerateOptions,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
pub struct GenerateOptions {
    #[arg(short, long)]
    pub from: Option<String>,

    #[arg(long, default_value_t = false)]
    pub force: bool,

    #[arg(short, long, default_value_t = BuildSystem::Cmake)]
    pub build_system: BuildSystem,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BuildSystem {
    Cmake,
}

impl Display for BuildSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildSystem::Cmake => f.write_str("cmake"),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    New {
        out: PathBuf,
    },

    Init { },
}
