use serde::Serialize;

#[derive(Serialize)]
pub struct SourceAbstract<'a> {
    pub id: &'a str,
    pub pos: &'a str,
    pub name: &'a str,
    pub file: &'a str,
    pub pkg: &'a str,
    pub comment: &'a str,
}

#[derive(Serialize)]
pub struct SourceReference<'a> {
    pub id: &'a str,
    pub pos: &'a str,
    pub name: &'a str,
}
