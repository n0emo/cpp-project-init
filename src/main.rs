// TODO: Bake some default templates into executable
//         - [ ] Minimal
//         - [ ] Gtest
//         - [ ] Raylib
//
// TODO: Unit tests
//         - [ ] Collect test coverage
//         - [ ] Unit tests for Cmake generators
//
// TODO: Init command
//
// TODO: Improve libraries generation
//         - [ ] Do not create lib directory if it contains only CMakeLists.txt
//         - [ ] External packages
//         - [ ] Prefetch packages
//
// TODO: Read templates from config directories
//         - [ ] Read from system configs
//         - [ ] Read from XDG-something user configs
//
// TODO: Catch2 testing backend
//
// TODO: README.md
//
// TODO: More backend support
//         - [ ] Xmake backend
//         - [ ] Meson backend
//         - [ ] Allow per-backend tweaks in config
//
// TODO: C language support
//
// TODO: GUI
//         - [ ] Decide the framework
//         - [ ] Split project to lib and cli executable
//         - [ ] GUI executable
use std::fs;

use clap::Parser;
use cli::Cli;
use generators::{CmakeProject, Generator};
use project::Project;

pub(crate) mod cli;
pub(crate) mod project;
pub(crate) mod generators;
pub(crate) mod tree;
pub(crate) mod strings;

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
                cli::BuildSystem::Cmake => CmakeProject::generate(project, &out)?,
            }
        }

        cli::Command::Init {} => {
            todo!()
        }
    }

    Ok(())
}
