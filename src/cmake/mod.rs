use std::{fs, path::Path};

use project::{FetchPackage, SrcDir, Testing};
use templates::*;

use crate::utils;

mod templates;
mod project;

pub fn generate(project: crate::project::Project, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let project = project::Project::try_from(project)?;
    let out_dir = out_dir.as_ref();

    let mut subdirectories = Vec::new();
    subdirectories.push(project.src.dir.clone());
    if !project.fetch_packages.is_empty() {
        subdirectories.push("lib".into());
    }

    if let Some(ref testing) = project.testing {
        subdirectories.push(testing.dir.clone());
    }

    let template = CmakeTemplate {
        name: project.project_name.clone(),
        subdirectories,
        enable_testing: project.testing.is_some()
    };
    utils::render_to(template, out_dir.join("CMakeLists.txt"))?;

    generate_src(project.src, out_dir)?;

    if !project.fetch_packages.is_empty() {
        generate_lib(project.fetch_packages, out_dir)?;
    }

    if let Some(testing) = project.testing {
        generate_tests(project.project_name.clone(), testing, out_dir)?;
    }

    Ok(())
}

fn generate_src(src: SrcDir, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let src_path = out_dir.as_ref().join(&src.dir);
    fs::create_dir_all(&src_path)?;

    let template = CmakeSrcTemplate {
        targets: src.targets,
    };
    utils::render_to(template, src_path.join("CMakeLists.txt"))?;

    for (name, contents) in src.files {
        fs::write(src_path.join(name), contents)?;
    }
    Ok(())
}

fn generate_tests(name: String, testing: Testing, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let dir = out_dir.as_ref().join(&testing.dir);
    fs::create_dir_all(&dir)?;

    let sources = testing.files.iter().map(|(n, _)| n).cloned().collect();

    let template = CmakeTestsTemplate { name, sources };
    utils::render_to(template, dir.join("CMakeLists.txt"))?;

    for (name, contents) in testing.files {
        fs::write(dir.join(name), contents)?;
    }

    Ok(())
}

fn generate_lib(fetch_packages: Vec<FetchPackage>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let lib_path = out_dir.as_ref().join("lib");
    fs::create_dir_all(&lib_path)?;

    let template = CmakeLibTemplate { fetch_packages };
    utils::render_to(template, lib_path.join("CMakeLists.txt"))?;
    Ok(())
}
