use std::collections::HashSet;

use crate::gentoo::{
    InstalledPackage,
    package::{Package, PackageKey},
};

#[derive(Debug, Default)]
pub struct Portage {
    pub installed: Vec<InstalledPackage>,
    pub world_set: HashSet<PackageKey>,
    pub available: Vec<Package>,
}

impl Portage {
    // TODO: Do this differently to allow backgrounded loading
    pub fn new(
        installed: Vec<InstalledPackage>,
        world_set: HashSet<PackageKey>,
        available: Vec<Package>,
    ) -> Self {
        Self {
            installed,
            world_set,
            available,
        }
    }

    pub fn installed_packages(&self) -> Vec<&InstalledPackage> {
        self.installed.iter().collect()
    }

    pub fn world_packages_len(&self) -> usize {
        self.world_set.len()
    }

    pub fn world_packages(&self) -> Vec<&InstalledPackage> {
        self.installed
            .iter()
            .filter(|pkg| self.world_set.contains(&pkg.atom))
            .collect()
    }

    pub fn get_installed_package(&self, key: &PackageKey) -> Option<&InstalledPackage> {
        let pkg = self.installed.iter().find(|pkg| pkg.atom == *key);
        if let Some(pkg) = pkg {
            return Some(pkg);
        }

        None
    }

    pub fn available_packages(&self) -> Vec<&Package> {
        let mut pkg: Vec<&Package> = self.available.iter().collect();

        pkg.sort_by_key(|p| &p.atom);

        pkg
    }

    pub fn get_available_package(&self, key: &PackageKey) -> Option<&Package> {
        let pkg = self.available.iter().find(|pkg| pkg.atom == *key);
        if let Some(pkg) = pkg {
            return Some(pkg);
        }

        None
    }

    pub fn total_installed_size(&self) -> usize {
        self.installed.iter().map(|p| p.size).sum()
    }

    pub(crate) fn installed_packages_len(&self) -> usize {
        self.installed.len()
    }
}
