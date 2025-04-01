use serde::Serialize;

#[derive(Serialize)]
pub struct SourcePkg<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub path: &'a str,
}
