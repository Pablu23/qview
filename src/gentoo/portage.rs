use std::fs;

use crate::gentoo::{Package, UseFlag};

#[derive(Debug)]
pub struct Portage {
    pub installed_packages: Vec<Package>,
}

fn split_pkg(input: &str) -> (&str, Option<&str>) {
    let bytes = input.as_bytes();

    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'-' {
            if let Some(next) = bytes.get(i + 1) {
                if next.is_ascii_digit() {
                    let name = &input[..i];
                    let version = &input[i + 1..];
                    return (name, Some(version));
                }
            }
        }
    }

    (input, None)
}

impl Portage {
    pub fn new() -> Self {
        Self {
            installed_packages: vec![],
        }
    }

    pub fn load_installed_packages(&mut self) -> Result<(), std::io::Error> {
        let categories = fs::read_dir("/var/db/pkg")?;

        for category in categories {
            let category = category.unwrap();
            let cat_name = category.file_name();
            let pkgs = fs::read_dir(category.path())?;
            for pkg in pkgs {
                let pkg = pkg.unwrap();
                let pkg_name = pkg.file_name();
                let pkg_name = pkg_name.to_str().unwrap();
                let (pkg_name, pkg_version) = split_pkg(pkg_name);

                let use_flags_file = fs::read(pkg.path().join("USE"))?;
                let use_flags: Vec<&str> = str::from_utf8(&use_flags_file)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .collect();

                let iuse_flags_file = fs::read(pkg.path().join("IUSE"))?;
                let iuse_flags: Vec<&str> = str::from_utf8(&iuse_flags_file)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .collect();

                let use_flags: Vec<UseFlag> = iuse_flags
                    .iter()
                    .filter(|x| !x.trim().is_empty())
                    .map(|&iuse| {
                        let is_default = iuse.starts_with("+");
                        let iuse = iuse.strip_prefix("+").unwrap_or(iuse);

                        let is_active = use_flags.contains(&iuse);

                        UseFlag::new(iuse.into(), is_active, is_default)
                    })
                    .collect();

                self.installed_packages.push(Package::new(
                    format!("{}/{}", cat_name.to_str().unwrap(), pkg_name),
                    pkg_version.unwrap().into(),
                    use_flags,
                ));
            }
        }

        Ok(())
    }
}
