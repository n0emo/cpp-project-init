use std::{fmt::Write as _, io};

use crate::{project::Project, tree::SourceNode};

#[derive(Default)]
pub struct FetchPackage {
    pub name: String,
    pub url: Option<String>,
    pub checksum: Option<String>,
    pub git_url: Option<String>,
    pub git_tag: Option<String>,
}

pub struct CmakeLibs {
    fetch_packages: Vec<FetchPackage>,
}

impl CmakeLibs {
    pub fn from_project(project: &Project) -> Result<Option<Self>, io::Error> {
        if project.packages.is_empty() {
            return Ok(None);
        }

        let fetch_packages = project
            .packages
            .iter()
            .filter_map(|p| {
                if let crate::project::PackageSource::Git { url, tag } = p.1 {
                    Some(FetchPackage {
                        name: p.0.clone(),
                        git_url: Some(url.clone()),
                        git_tag: tag.clone(),
                        ..Default::default()
                    })
                } else if let crate::project::PackageSource::Download { url, checksum } = p.1 {
                    Some(FetchPackage {
                        name: p.0.clone(),
                        url: Some(url.clone()),
                        checksum: checksum.clone(),
                        ..Default::default()
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(Some(Self { fetch_packages }))
    }

    pub fn into_node(self) -> Result<SourceNode, anyhow::Error> {
        let mut sb = String::new();
        write_libs(&mut sb, &self.fetch_packages)?;

        Ok(SourceNode::Directory {
            name: "lib".into(),
            children: vec![SourceNode::File {
                name: "CMakeLists.txt".into(),
                contents: sb.into_bytes(),
            }],
        })
    }
}

fn write_libs(sb: &mut String, fetch_packages: &Vec<FetchPackage>) -> Result<(), anyhow::Error> {
    writeln!(sb, "include(FetchContent)")?;
    writeln!(sb)?;
    for package in fetch_packages {
        write_package(sb, package)?;
    }
    writeln!(sb, r#"message(STATUS "Fetching packages")"#)?;
    writeln!(
        sb,
        "FetchContent_MakeAvailable({packages})",
        packages = fetch_packages
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(" "),
    )?;

    Ok(())
}

fn write_package(sb: &mut String, package: &FetchPackage) -> Result<(), anyhow::Error> {
    writeln!(sb, "FetchContent_Declare(")?;
    writeln!(sb, "    {}", package.name)?;
    if let Some(url) = &package.url {
        writeln!(sb, "    URL {url}")?;
    }
    if let Some(hash) = &package.checksum {
        writeln!(sb, "    URL_HASH {hash}")?;
    }
    if let Some(git_url) = &package.git_url {
        writeln!(sb, "    GIT_REPOSITORY {git_url}")?;
    }
    if let Some(git_tag) = &package.git_tag {
        writeln!(sb, "    GIT_TAG {git_tag}")?;
    }
    writeln!(sb, ")")?;
    writeln!(sb)?;

    Ok(())
}
