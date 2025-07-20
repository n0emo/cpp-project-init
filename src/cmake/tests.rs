use std::{fs, path::Path};
use std::fmt::Write as _;

use indoc::writedoc;

use super::project::Testing;

pub fn generate(name: String, testing: Testing, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let dir = out_dir.as_ref().join(&testing.dir);

    let sources = testing.files.iter().map(|(n, _)| n).cloned().collect();

    let mut sb = String::new();

    write_tests(&mut sb, &name, &sources)?;

    fs::create_dir_all(&dir)?;
    fs::write(dir.join("CMakeLists.txt"), &sb)?;

    for (name, contents) in testing.files {
        fs::write(dir.join(name), contents)?;
    }

    Ok(())
}

fn write_tests(sb: &mut String, name: &str, sources: &Vec<String>) -> anyhow::Result<()> {
    writeln!(sb, "add_executable({name}_test")?;
    for source in sources {
        writeln!(sb, "    {source}")?;
    }
    writeln!(sb, ")")?;
    writeln!(sb, "")?;

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

