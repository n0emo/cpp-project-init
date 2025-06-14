use askama::Template;

use super::{project::Target, FetchPackage};

#[derive(Template)]
#[template(path = "cmake/main.txt")]
pub struct CmakeTemplate {
    pub name: String,
    pub subdirectories: Vec<String>,
    pub enable_testing: bool,
}

#[derive(Template)]
#[template(path = "cmake/src.txt")]
pub struct CmakeSrcTemplate {
    pub targets: Vec<Target>
}

#[derive(Template)]
#[template(path = "cmake/tests.txt")]
pub struct CmakeTestsTemplate {
    pub name: String,
    pub sources: Vec<String>,
}

#[derive(Template)]
#[template(path = "cmake/lib.txt")]
pub struct CmakeLibTemplate {
   pub fetch_packages: Vec<FetchPackage>,
}
