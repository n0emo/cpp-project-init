use std::{fmt::Write as _, fs, path::Path};

use indoc::writedoc;

pub struct MainCmakeLists<'a> {
    pub out: &'a Path,
    pub name: &'a str,
    pub subdirectories: &'a Vec<String>,
    pub enable_testing: bool,
}

impl<'a> MainCmakeLists<'a> {
    pub fn generate(&self) -> anyhow::Result<()> {
        let mut sb = String::new();
        write_main(&mut sb, self.name, self.subdirectories, self.enable_testing)?;

        fs::write(self.out.join("CMakeLists.txt"), sb.trim())?;

        Ok(())
    }
}

fn write_main(sb: &mut String, name: &str, subdirectories: &Vec<String>, enable_testing: bool) -> anyhow::Result<()> {
    writedoc!(
        sb,
        r"
        cmake_minimum_required(VERSION 3.10)

        set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

        project({name})
        "
    )?;
    writeln!(sb, "")?;

    if enable_testing {
        writeln!(sb, "enable_testing()")?;
        writeln!(sb, "")?;
    }

    for dir in subdirectories {
        writeln!(sb, "add_subdirectory({dir})")?;
    }

    Ok(())
}
