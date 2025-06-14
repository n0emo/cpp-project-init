use std::fs;

pub(super) struct Project {
    pub project_name: String,
    pub testing: Option<Testing>,
    pub src: SrcDir,
    pub fetch_packages: Vec<FetchPackage>,
}

impl TryFrom<crate::project::Project> for Project {
    type Error = anyhow::Error;

    fn try_from(value: crate::project::Project) -> Result<Self, Self::Error> {
        let fetch_packages = value
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

        let main_file =
            load_source_file(value.src.main_file.name.clone(), value.src.main_file.source)?;

        let sources = value
            .src
            .sources
            .into_iter()
            .map(|(n, s)| load_source_file(n, s))
            .collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

        let headers = value
            .src
            .headers
            .into_iter()
            .map(|(n, s)| load_source_file(n, s))
            .collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

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
                sources: sources.into_iter().map(|s| s.0).collect(),
            });
        }
        link_libraries.extend(value.src.libraries);

        targets.push(Target::Exe {
            name: value.name.clone(),
            sources: vec![main_file.0],
            link_libraries,
        });

        let testing = value
            .testing
            .map(|t| {
                let files = t
                    .sources
                    .into_iter()
                    .map(|(name, source)| load_source_file(name.clone(), source))
                    .collect::<Result<_, _>>()?;

                Ok::<Testing, anyhow::Error>(Testing {
                    dir: t.dir,
                    framework: t.framework,
                    files,
                })
            })
            .transpose()?;

        Ok(Self {
            project_name: value.name.clone(),

            testing,

            src: SrcDir {
                dir: value.src.dir,
                files,
                targets,
            },

            fetch_packages,
        })
    }
}

fn load_source_file(
    name: String,
    source: crate::project::SourceFile,
) -> anyhow::Result<(String, String)> {
    Ok((
        name,
        match source {
            crate::project::SourceFile::Path(path) => fs::read_to_string(path)?,
            crate::project::SourceFile::Contents(contents) => contents,
        },
    ))
}

pub(super) struct Testing {
    pub dir: String,
    #[allow(unused, reason = "Supporting only GoogleTest for now")]
    pub framework: crate::project::TestingFramework,
    pub files: Vec<(String, String)>,
}

pub(super) struct SrcDir {
    pub dir: String,
    pub files: Vec<(String, String)>,
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

#[derive(Default)]
pub(super) struct FetchPackage {
    pub name: String,
    pub url: Option<String>,
    pub checksum: Option<String>,
    pub git_url: Option<String>,
    pub git_tag: Option<String>,
}
