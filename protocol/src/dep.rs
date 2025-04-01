use serde::Serialize;

#[derive(Serialize)]
pub struct SourceDep<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub typ: &'a str,
    pub rf: &'a str,
}
