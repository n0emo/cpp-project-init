use std::{fs, path::{Path, PathBuf}};

#[derive(Default)]
pub struct SourceTree {
    pub children: Vec<SourceNode>,
}

impl SourceTree {
    pub fn render(&self, path: &Path) -> std::io::Result<()> {
        let mut stack = vec![path];

        for child in &self.children {
            child.render(&mut stack)?;
        }

        Ok(())
    }
}

pub enum SourceNode {
    Directory {
        name: PathBuf,
        children: Vec<SourceNode>
    },
    File {
        name: PathBuf,
        contents: Vec<u8>,
    }
}

impl SourceNode {
    fn render<'a>(&'a self, stack: &mut Vec<&'a Path>) -> std::io::Result<()> {
        match self {
            SourceNode::Directory { name, children } => {
                stack.push(name);
                let path = PathBuf::from_iter(stack.iter());
                fs::create_dir_all(&path)?;

                for child in children {
                    child.render(stack)?;
                }
                stack.pop();
            },

            SourceNode::File { name, contents } => {
                stack.push(name);
                let path = PathBuf::from_iter(stack.iter());
                fs::write(path, contents)?;
                stack.pop();
            },
        }

        Ok(())
    }
}
