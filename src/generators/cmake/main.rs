use std::fmt::Write as _;

use indoc::writedoc;

use crate::{project::Project, tree::SourceNode};

pub struct CmakeMain {
    pub name: String,
    pub subdirectories: Vec<String>,
    pub enable_testing: bool,
}

impl TryFrom<&Project> for CmakeMain {
    type Error = anyhow::Error;

    fn try_from(value: &Project) -> Result<Self, Self::Error> {
        let mut subdirectories = Vec::new();
        subdirectories.push(value.src.dir.clone());
        if !value.packages.is_empty() {
            subdirectories.push("lib".into());
        }

        if let Some(ref testing) = value.testing {
            subdirectories.push(testing.dir.clone());
        }

        Ok(Self {
            name: value.name.clone(),
            subdirectories,
            enable_testing: value.testing.is_some(),
        })
    }
}

impl TryFrom<CmakeMain> for SourceNode {
    type Error = std::fmt::Error;

    fn try_from(value: CmakeMain) -> Result<Self, Self::Error> {
        let mut sb = String::new();
        write_main(&mut sb, &value.name, &value.subdirectories, value.enable_testing)?;
        Ok(Self::File {
            name: "CMakeLists.txt".into(),
            contents: sb.into_bytes(),
        })
    }
}

fn write_main(sb: &mut String, name: &str, subdirectories: &Vec<String>, enable_testing: bool) -> std::fmt::Result {
    writedoc!(
        sb,
        r"
        cmake_minimum_required(VERSION 3.10)

        set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

        project({name})
        "
    )?;
    writeln!(sb)?;

    if enable_testing {
        writeln!(sb, "enable_testing()")?;
        writeln!(sb)?;
    }

    for dir in subdirectories {
        writeln!(sb, "add_subdirectory({dir})")?;
    }

    Ok(())
}
