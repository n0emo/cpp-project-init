use std::fmt::Write as _;

use crate::{project::Project, strings::trim_newline, tree::SourceNode};

use super::file::SourceFile;

pub(super) struct CmakeSrc {
    pub dir: String,
    pub files: Vec<SourceFile>,
    pub targets: Vec<Target>,
}

pub enum Target {
    Exe {
        name: String,
        sources: Vec<String>,
        link_libraries: Vec<String>,
    },

    Lib {
        sources: Vec<String>,
        name: String,
    },
}

impl TryFrom<&Project> for CmakeSrc {
    type Error = anyhow::Error;

    fn try_from(value: &Project) -> Result<Self, Self::Error> {
        let main_file = SourceFile::load(value.src.main_file.name.clone(), &value.src.main_file.source)?;

        let sources: Vec<SourceFile> = value
            .src
            .sources
            .iter()
            .map(|(n, s)| SourceFile::load(n.clone(), s))
            .collect::<Result<Vec<_>, _>>()?;

        let headers: Vec<SourceFile> = value
            .src
            .headers
            .iter()
            .map(|(n, s)| SourceFile::load(n.clone(), s))
            .collect::<Result<Vec<_>, _>>()?;

        let files = std::iter::once(&main_file)
            .chain(sources.iter())
            .chain(headers.iter())
            .cloned()
            .collect();

        let mut targets = Vec::new();
        let mut link_libraries = Vec::new();
        if !sources.is_empty() {
            let name = format!("{name}_lib", name = value.name);
            link_libraries.push(name.clone());
            targets.push(Target::Lib {
                name,
                sources: sources.into_iter().map(|s| s.name).collect(),
            });
        }
        link_libraries.extend(value.src.libraries.clone());

        targets.push(Target::Exe {
            name: value.name.clone(),
            sources: vec![main_file.name],
            link_libraries,
        });

        Ok(Self {
            dir: value.src.dir.clone(),
            files,
            targets,
        })
    }
}

impl TryFrom<CmakeSrc> for SourceNode {
    type Error = anyhow::Error;

    fn try_from(value: CmakeSrc) -> Result<Self, Self::Error> {
        let mut cmakelists = String::new();
        write_src(&mut cmakelists, &value.targets)?;
        trim_newline(&mut cmakelists);
        let mut children = Vec::new();
        children.push(SourceNode::File {
            name: "CMakeLists.txt".into(),
            contents: cmakelists.into_bytes(),
        });

        for file in value.files {
            children.push(file.into());
        }

        Ok(SourceNode::Directory { name: value.dir.into(), children })
    }
}

fn write_src(sb: &mut String, targets: &Vec<Target>) -> anyhow::Result<()> {
    writeln!(sb, "set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_BINARY_DIR}})")?;
    writeln!(sb)?;

    for target in targets {
        match target {
            Target::Exe { name, sources, link_libraries } => {
                writeln!(sb, "add_executable({name}")?;
                for source in sources {
                    writeln!(sb, "    {source}")?;
                }
                writeln!(sb, ")")?;
                writeln!(sb)?;

                if !link_libraries.is_empty() {
                    writeln!(sb, "target_link_libraries({name}")?;
                    for lib in link_libraries {
                        writeln!(sb, "    {lib}")?;
                    }
                    writeln!(sb, ")")?;
                    writeln!(sb)?;
                }
            },
            Target::Lib { sources, name } => {
                writeln!(sb, "add_library({name}")?;
                for source in sources {
                    writeln!(sb, "    {source}")?;
                }
                writeln!(sb, ")")?;
                writeln!(sb)?;
                writeln!(sb, "target_include_directories({name} PUBLIC .)")?;
                writeln!(sb)?;
            },
        }
    }

    Ok(())
}

