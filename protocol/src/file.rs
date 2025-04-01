use serde::Serialize;

#[derive(Serialize)]
pub struct SourceFile<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub path: &'a str,
    pub pkg: &'a str,
}
