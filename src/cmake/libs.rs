use std::{fmt::Write as _, fs, path::Path};

use super::project::FetchPackage;

pub fn generate(fetch_packages: Vec<FetchPackage>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut sb = String::new();
    write_libs(&mut sb, fetch_packages)?;

    let lib_path = out_dir.as_ref().join("lib");
    fs::create_dir_all(&lib_path)?;
    fs::write(lib_path.join("CMakeLists.txt"), &sb)?;

    Ok(())
}

fn write_libs(sb: &mut String, fetch_packages: Vec<FetchPackage>) -> Result<(), anyhow::Error> {
    writeln!(sb, "include(FetchContent)")?;
    writeln!(sb, "")?;
    for package in &fetch_packages {
        write_package(sb, package)?;
    }
    writeln!(sb, r#"message(STATUS "Fetching packages")"#)?;
    writeln!(
        sb,
        "FetchContent_MakeAvailable({packages})",
        packages = fetch_packages.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(" "),
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
    writeln!(sb, "")?;

    Ok(())
}

