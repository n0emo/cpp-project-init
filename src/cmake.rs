use std::path::Path;

use main::MainCmakeLists;

mod libs;
mod main;
mod project;
mod src;
mod tests;

pub fn generate(project: crate::project::Project, out: impl AsRef<Path>) -> anyhow::Result<()> {
    let project = project::Project::try_from(project)?;
    let out = out.as_ref();

    let mut subdirectories = Vec::new();
    subdirectories.push(project.src.dir.clone());
    if !project.fetch_packages.is_empty() {
        subdirectories.push("lib".into());
    }

    if let Some(ref testing) = project.testing {
        subdirectories.push(testing.dir.clone());
    }

    let main = MainCmakeLists {
        out,
        name: &project.project_name,
        subdirectories: &subdirectories,
        enable_testing: project.testing.is_some(),
    };
    main.generate()?;

    let src = src::CmakeSrc {
        out,
        src: &project.src,
    };
    src.generate()?;

    if !project.fetch_packages.is_empty() {
        libs::generate(project.fetch_packages, out)?;
    }

    if let Some(testing) = project.testing {
        tests::generate(project.project_name.clone(), testing, out)?;
    }

    Ok(())
}
