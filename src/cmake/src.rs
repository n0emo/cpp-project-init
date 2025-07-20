use std::{fmt::Write as _, fs, path::Path};

use super::project::{Target, SrcDir};

pub struct CmakeSrc<'a> {
    pub out: &'a Path,
    pub src: &'a SrcDir,
}

impl<'a> CmakeSrc<'a> {
    pub fn generate(self) -> anyhow::Result<()> {
        let src_path = self.out.join(&self.src.dir);

        let mut sb = String::new();
        write_src(&mut sb, &self.src.targets)?;

        fs::create_dir_all(&src_path)?;
        fs::write(src_path.join("CMakeLists.txt"), sb.trim())?;
        for (name, contents) in &self.src.files {
            fs::write(src_path.join(name), contents)?;
        }

        Ok(())
    }
}

fn write_src(sb: &mut String, targets: &Vec<Target>) -> anyhow::Result<()> {
    writeln!(sb, "set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_BINARY_DIR}})")?;
    writeln!(sb, "")?;

    for target in targets {
        match target {
            Target::Exe { name, sources, link_libraries } => {
                writeln!(sb, "add_executable({name}")?;
                for source in sources {
                    writeln!(sb, "    {source}")?;
                }
                writeln!(sb, ")")?;
                writeln!(sb, "")?;

                if !link_libraries.is_empty() {
                    writeln!(sb, "target_link_libraries({name}")?;
                    for lib in link_libraries {
                        writeln!(sb, "    {lib}")?;
                    }
                    writeln!(sb, ")")?;
                    writeln!(sb, "")?;
                }
            },
            Target::Lib { sources, name } => {
                writeln!(sb, "add_library({name}")?;
                for source in sources {
                    writeln!(sb, "    {source}")?;
                }
                writeln!(sb, ")")?;
                writeln!(sb, "")?;
                writeln!(sb, "target_include_directories({name} PUBLIC .)")?;
                writeln!(sb, "")?;
            },
        }
    }

    Ok(())
}

