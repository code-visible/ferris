pub struct File {
    id: String,
    name: String,
    path: String,
    pkg: String,
}

impl File {
    pub fn new() -> Self {
        File {
            id: String::new(),
            name: String::new(),
            path: String::new(),
            pkg: String::new(),
        }
    }

    pub fn parse(&self) {}
}
