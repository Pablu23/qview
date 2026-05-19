use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::gentoo::UseFlag;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PackageKey {
    pub category: String,
    pub name: String,
}

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

    pub versions: Vec<Version>,
}

#[derive(Debug, Clone)]
pub struct Version {
    pub version: String,

    pub metadata: Metadata,

    pub keywords: Vec<String>,
    pub iuse: Vec<UseFlag>,
}
