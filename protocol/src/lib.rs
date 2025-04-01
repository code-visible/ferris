pub mod call;
pub mod definition;
pub mod dep;
pub mod file;
pub mod pkg;

use call::{SourceCall, SourceFunction};
use definition::{SourceAbstract, SourceReference};
use dep::SourceDep;
use file::SourceFile;
use pkg::SourcePkg;
use serde::Serialize;

#[derive(Serialize)]
pub struct Project<'a: 'b, 'b> {
    pub name: &'b str,
    pub lang: &'b str,
    pub parser: &'b str,
    pub timestamp: &'b str,
    pub repository: &'b str,
    pub version: &'b str,
    pub typ: &'b str,
    pub pkgs: Vec<SourcePkg<'a>>,
    pub files: Vec<SourceFile<'a>>,
    pub abstracts: Vec<SourceAbstract<'a>>,
    pub callables: Vec<SourceFunction<'a>>,
    pub calls: Vec<SourceCall<'a>>,
    pub refs: Vec<SourceReference<'a>>,
    pub deps: Vec<SourceDep<'a>>,
}
