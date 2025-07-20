use libs::CmakeLibs;
use main::CmakeMain;
use src::CmakeSrc;
use testing::Testing;

use crate::{project::Project, tree::SourceTree};

use super::Generator;

mod libs;
mod main;
mod src;
mod file;
mod testing;

pub struct CmakeProject {
    main: CmakeMain,
    src: CmakeSrc,
    libs: Option<CmakeLibs>,
    testing: Option<Testing>,
}

impl TryFrom<Project> for CmakeProject {
    type Error = anyhow::Error;

    fn try_from(value: Project) -> Result<Self, Self::Error> {
        Ok(Self {
            main: CmakeMain::try_from(&value)?,
            src: CmakeSrc::try_from(&value)?,
            libs: CmakeLibs::from_project(&value)?,
            testing: Testing::from_project(&value)?,
        })
    }
}

impl Generator for CmakeProject {
    fn into_tree(self) -> anyhow::Result<SourceTree> {
        let mut children = vec![
            self.main.try_into()?,
            self.src.try_into()?,
        ];

        if let Some(libs) = self.libs {
            children.push(libs.into_node()?);
        }

        if let Some(testing) = self.testing {
            children.push(testing.into_node()?);
        }

        Ok(SourceTree { children })
    }
}
