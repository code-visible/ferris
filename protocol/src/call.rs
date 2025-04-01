use serde::Serialize;

#[derive(Serialize)]
pub struct SourceFunction<'a> {
    pub id: &'a str,
    pub pos: &'a str,
    pub name: &'a str,
    pub file: &'a str,
    pub pkg: &'a str,
    pub comment: &'a str,
}

#[derive(Serialize)]
pub struct SourceCall<'a> {
    pub id: &'a str,
    pub pos: &'a str,
    pub caller: &'a str,
    pub callee: &'a str,
    pub file: &'a str,
}
