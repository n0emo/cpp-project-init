use std::{fs, path::Path};

use askama::Template;

pub fn render(template: impl Template) -> anyhow::Result<String> {
    let mut str = template.render()?.trim().to_owned();
    str.push('\n');
    Ok(str)
}

pub fn render_to(template: impl Template, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    let str = render(template)?;
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, &str)?;
    Ok(())
}
