pub mod call;
pub mod definition;
pub mod dep;
pub mod file;
pub mod pkg;

use call::{SourceCall, SourceFunction};
use definition::SourceAbstract;
use dep::SourceDep;
use file::SourceFile;
use pkg::SourcePkg;

pub struct Project {
    name: string,
    directory: string,
    pkgs: Vec<SourcePkg>,
    files: Vec<SourceFile>,
    abstracts: Vec<SourceAbstract>,
    callables: Vec<SourceFunction>,
    calls: Vec<SourceCall>,
    deps: Vec<SourceDep>,
}
