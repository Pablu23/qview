use std::collections::HashSet;

use crate::gentoo::UseFlag;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Package {
    pub name: String,
    pub use_flags: HashSet<UseFlag>,
    pub version: String,
    pub repository: String,
    pub maintainer: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<Vec<String>>,
    pub license: Option<String>,
    pub size: usize,
}

// #[allow(dead_code)]
// impl Package {
//     pub fn new(
//         name: String,
//         version: String,
//         repository: String,
//         size: usize,
//         homepage: Option<Vec<String>>,
//         license: Option<String>,
//         description: Option<String>,
//         maintainer: Option<String>,
//         use_flags: HashSet<UseFlag>,
//     ) -> Self {
//         Package {
//             name,
//             use_flags,
//             version,
//             repository,
//             maintainer,
//             description,
//             homepage,
//             license,
//             size,
//         }
//     }
// }
