use std::path::Path;

use crate::{project::Project, tree::SourceTree};

mod cmake;

pub use cmake::CmakeProject;

pub trait Generator: TryFrom<Project, Error=anyhow::Error> {
    fn into_tree(self) -> anyhow::Result<SourceTree>;

    fn generate(project: Project, out: &Path) -> anyhow::Result<()> {
        Self::try_from(project)?.into_tree()?.render(out)?;

        Ok(())
    }
}
