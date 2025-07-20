use std::fs;

use crate::tree::SourceNode;

#[derive(Clone)]
pub struct SourceFile {
    pub name: String,
    pub contents: String,
}

impl SourceFile {
    pub fn load(name: String, source: &crate::project::SourceFile) -> std::io::Result<Self> {
        let contents = match source {
            crate::project::SourceFile::Path(path) => fs::read_to_string(path)?,
            crate::project::SourceFile::Contents(contents) => contents.clone(),
        };

        Ok(SourceFile { name, contents })
    }
}

impl From<SourceFile> for SourceNode {
    fn from(value: SourceFile) -> Self {
        Self::File {
            name: value.name.clone().into(),
            contents: value.contents.clone().into_bytes(),
        }
    }
}
