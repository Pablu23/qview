use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::gentoo::UseFlag;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackageKey {
    // Field declaration matters because Ord, and PartialOrd check in order
    pub category: String,
    pub name: String,
}

// impl Ord for PackageKey {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let cat_ord = self.category.cmp(&other.category);
//         if cat_ord == Ordering::Equal {
//             return self.name.cmp(&other.name);
//         }
//
//         cat_ord
//     }
// }
//
// impl PartialOrd for PackageKey {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match self.category.partial_cmp(&other.category) {
//             Some(core::cmp::Ordering::Equal) => {}
//             ord => return ord,
//         }
//         self.name.partial_cmp(&other.name)
//     }
// }

impl PackageKey {
    pub fn from_atom(s: &str) -> Option<Self> {
        let (category, name) = s.split_once('/')?;
        Some(Self {
            category: category.to_string(),
            name: name.to_string(),
        })
    }

    pub fn qualified_name(&self) -> String {
        format!("{}/{}", self.category, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub maintainer: Option<String>,
    pub description: Option<String>,
    pub homepage: Vec<String>,
    pub license: Option<String>,
    pub repository: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct InstalledPackage {
    pub atom: PackageKey,

    pub version: String,

    pub build_time: DateTime<Utc>,

    pub slot: i32,

    pub metadata: Metadata,

    pub enabled_use_flags: HashSet<String>,
    pub iuse: Vec<UseFlag>,

    pub size: usize,
}

#[derive(Debug)]
pub struct Package {
    pub atom: PackageKey,

    pub versions: Vec<PackageVersion>,
}

#[derive(Debug, Clone)]
pub struct PackageVersion {
    pub version: String,

    pub metadata: Metadata,

    pub keywords: Vec<String>,
    pub iuse: Vec<UseFlag>,
}
