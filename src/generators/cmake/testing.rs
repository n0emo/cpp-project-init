use std::path::PathBuf;
use std::fmt::{self, Write as _};

use indoc::writedoc;

use crate::project::Project;
use crate::tree::SourceNode;

use super::file::SourceFile;

pub struct Testing {
    pub dir: PathBuf,
    pub project_name: String,
    #[allow(unused, reason = "Supporting only GoogleTest for now")]
    pub framework: crate::project::TestingFramework,
    pub files: Vec<SourceFile>,
}

impl Testing {
    pub fn from_project(project: &Project) -> std::io::Result<Option<Self>> {
        project
            .testing
            .as_ref()
            .map(|t| {
                let files = t
                    .sources
                    .iter()
                    .map(|(name, source)| SourceFile::load(name.clone(), source))
                    .collect::<Result<_, _>>()?;

                Ok(Testing {
                    dir: t.dir.as_str().into(),
                    framework: t.framework,
                    project_name: project.name.clone(),
                    files,
                })
            })
            .transpose()
    }

    pub fn into_node(self) -> Result<SourceNode, fmt::Error> {
        let sources = self.files.iter().map(|s| &s.name).cloned().collect();
        let mut cmakelists = String::new();
        write_tests(&mut cmakelists, &self.project_name, &sources)?;

        let mut children = vec![
            SourceNode::File { name: "CMakeLists.txt".into(), contents: cmakelists.into_bytes() },
        ];

        for file in self.files {
            children.push(file.into());
        }

        Ok(SourceNode::Directory { name: self.dir, children })
    }
}

fn write_tests(sb: &mut String, name: &str, sources: &Vec<String>) -> fmt::Result {
    writeln!(sb, "add_executable({name}_test")?;
    for source in sources {
        writeln!(sb, "    {source}")?;
    }
    writeln!(sb, ")")?;
    writeln!(sb)?;

    writedoc!(
        sb,
        r"
        target_link_libraries({name}_test
            {name}_lib
            gtest
            gtest_main
        )

        include(GoogleTest)
        gtest_discover_tests({name}_test)
        "

    )?;

    Ok(())
}

