use core::package::NameVer;

#[deriving(Eq,Clone,Show)]
pub struct Dependency {
    name: NameVer
}

impl Dependency {
    pub fn new(name: &str) -> Dependency {
        Dependency { name: NameVer::new(name.to_owned(), "1.0.0") }
    }

    pub fn with_name_and_version(name: &str, version: &str) -> Dependency {
        Dependency { name: NameVer::new(name, version) }
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        self.name.get_name()
    }
}
