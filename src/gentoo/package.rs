use crate::gentoo::UseFlag;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Package {
    pub name: String,
    pub use_flags: Vec<UseFlag>,
    pub version: String,
}

#[allow(dead_code)]
impl Package {
    pub fn new(name: String, version: String, use_flags: Vec<UseFlag>) -> Self {
        Package {
            name: name,
            use_flags: use_flags,
            version: version,
        }
    }
}
