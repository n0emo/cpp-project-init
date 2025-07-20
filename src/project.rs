use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(default = "Project::default_name")]
    pub name: String,

    #[serde(default)]
    pub testing: Option<Testing>,

    #[serde(default)]
    pub src: SrcDir,

    #[serde(default)]
    pub packages: HashMap<String, PackageSource>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: Self::default_name(),
            testing: Default::default(),
            src: Default::default(),
            packages: Default::default(),
        }
    }
}

impl Project {
    fn default_name() -> String {
        "app".into()
    }

    pub fn load(input: impl AsRef<Path>) -> anyhow::Result<Self> {
        let input = input.as_ref();

        let not_found =
            || anyhow::anyhow!("Project description file not found in the description directory");

        let file = if !input.is_dir() {
            input.to_path_buf()
        } else {
            let files: Vec<PathBuf> = fs::read_dir(input)?
                .filter_map(|e| Some(e.ok()?.path()))
                .filter(|p| {
                    if let Some(f) = p.file_stem() {
                        f == OsStr::new("project")
                    } else {
                        false
                    }
                })
                .collect();

            match files.as_slice() {
                [] => bail!(not_found()),
                [file] => file.clone(),
                _ => bail!("More than one description file found in the description directory"),
            }
        };

        let ext = file
            .extension()
            .ok_or_else(not_found)?
            .to_str()
            .ok_or_else(not_found)?;

        let contents = fs::read_to_string(&file)?;

        let project = match ext {
            "yaml" | "yml" => serde_yml::from_str(&contents)?,
            "json" => serde_json::from_str(&contents)?,
            "toml" => toml::from_str(&contents)?,
            "xml" => serde_xml_rs::from_str(&contents)?,
            _ => bail!(not_found()),
        };

        Ok(project)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Testing {
    #[serde(default = "default_tests_dir")]
    pub dir: String,

    #[serde(default)]
    pub framework: TestingFramework,

    #[serde(default = "default_tests_sources")]
    pub sources: HashMap<String, SourceFile>,
}

fn default_tests_dir() -> String {
    "tests".into()
}

fn default_tests_sources() -> HashMap<String, SourceFile> {
    HashMap::from_iter([(
        "test_greet.cpp".into(),
        SourceFile::Contents(include_str!("default/sources/test_greet_gtest.cpp").into()),
    )])
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub enum TestingFramework {
    #[default]
    GoogleTest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SrcDir {
    #[serde(default = "default_src_dir")]
    pub dir: String,

    #[serde(default)]
    pub main_file: MainFile,

    #[serde(default)]
    pub sources: HashMap<String, SourceFile>,

    #[serde(default)]
    pub headers: HashMap<String, SourceFile>,

    #[serde(default)]
    pub libraries: Vec<String>,
}

impl Default for SrcDir {
    fn default() -> Self {
        Self {
            dir: default_src_dir(),
            sources: HashMap::from_iter([(
                "lib.cpp".to_owned(),
                SourceFile::Contents(include_str!("default/sources/lib.cpp").to_owned()),
            )]),
            headers: HashMap::from_iter([(
                "lib.hpp".to_owned(),
                SourceFile::Contents(include_str!("default/sources/lib.hpp").to_owned()),
            )]),
            main_file: Default::default(),
            libraries: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFile {
    Path(String),
    Contents(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainFile {
    #[serde(default = "default_main_name")]
    pub name: String,

    #[serde(flatten)]
    pub source: SourceFile,
}

fn default_main_name() -> String {
    "main.cpp".into()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageSource {
    Download {
        url: String,
        checksum: Option<String>,
    },
    Git {
        url: String,
        tag: Option<String>,
    },
}

impl Default for MainFile {
    fn default() -> Self {
        Self {
            name: default_main_name(),
            source: SourceFile::Contents(include_str!("default/sources/main.cpp").into()),
        }
    }
}

fn default_src_dir() -> String {
    "src".into()
}
