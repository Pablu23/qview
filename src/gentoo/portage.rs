use std::collections::HashSet;

use crate::gentoo::{
    InstalledPackage,
    package::{Package, PackageKey},
};

#[derive(Debug, Default)]
pub struct Portage {
    installed_packages: Vec<InstalledPackage>,
    world_packages: HashSet<PackageKey>,
    available_packages: Vec<Package>,
}

impl Portage {
    // TODO: Do this differently to allow backgrounded loading
    pub fn new(
        installed_packages: Vec<InstalledPackage>,
        world_packages: HashSet<PackageKey>,
        available_packages: Vec<Package>,
    ) -> Self {
        Self {
            installed_packages,
            world_packages,
            available_packages,
        }
    }

    pub fn installed_packages(&self) -> Vec<&InstalledPackage> {
        self.installed_packages.iter().collect()
    }

    pub fn world_packages_len(&self) -> usize {
        self.world_packages.len()
    }

    pub fn world_packages(&self) -> Vec<&InstalledPackage> {
        self.installed_packages
            .iter()
            .filter(|pkg| self.world_packages.contains(&pkg.atom))
            .collect()
    }

    pub fn get_installed_package(&self, index: usize) -> Option<&InstalledPackage> {
        if self.installed_packages.len() > index && index > 0 {
            return Some(&self.installed_packages[index]);
        }

        None
    }

    pub fn get_installed_package_key(&self, key: &PackageKey) -> Option<&InstalledPackage> {
        let pkg = self.installed_packages.iter().find(|pkg| pkg.atom == *key);
        if let Some(pkg) = pkg {
            return Some(pkg);
        }

        None
    }

    pub fn total_installed_size(&self) -> usize {
        self.installed_packages.iter().map(|p| p.size).sum()
    }

    pub(crate) fn installed_packages_len(&self) -> usize {
        self.installed_packages.len()
    }
}
