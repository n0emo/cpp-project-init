use std::fs;

use clap::Parser;
use cli::Cli;
use project::Project;

pub(crate) mod cli;
pub(crate) mod cmake;
pub(crate) mod project;
pub(crate) mod utils;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Command::New { out } => {
            let project = if let Some(from) = cli.generate.from {
                Project::load(from)?
            } else {
                Project::default()
            };

            if cli.generate.force && fs::exists(&out).unwrap_or(false) {
                fs::remove_dir_all(&out)?;
            }
            fs::create_dir_all(&out)?;

            match cli.generate.build_system {
                cli::BuildSystem::Cmake => {
                    cmake::generate(project, out)?;
                }
            }
        }

        cli::Command::Init {} => {
            todo!()
        }
    }

    Ok(())
}
