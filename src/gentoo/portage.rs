use std::collections::HashSet;

use crate::gentoo::{
    InstalledPackage,
    package::{Package, PackageKey},
};

#[derive(Debug, Default)]
pub struct Portage {
    pub installed_packages: Vec<InstalledPackage>,
    pub world_packages: HashSet<PackageKey>,
    pub available_packages: Vec<Package>,
}

impl Portage {
    pub fn new() -> Self {
        Self {
            installed_packages: vec![],
            world_packages: HashSet::new(),
            available_packages: vec![],
        }
    }

    pub fn get_installed_package(&self, key: PackageKey) -> Option<InstalledPackage> {
        let pkg = self.installed_packages.iter().find(|pkg| pkg.atom == key);
        if let Some(pkg) = pkg {
            return Some(pkg.clone());
        }

        None
    }

    pub fn total_installed_size(&self) -> usize {
        self.installed_packages.iter().map(|p| p.size).sum()
    }
}
