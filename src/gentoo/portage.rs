use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

use quick_xml::{Reader, events::Event};

use crate::gentoo::{Package, UseFlag};

#[derive(Debug, Default)]
pub struct Portage {
    pub installed_packages: Vec<Package>,
    pub world_packages: Vec<String>,
}

fn split_pkg(input: &str) -> (&str, Option<&str>) {
    let bytes = input.as_bytes();

    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'-'
            && let Some(next) = bytes.get(i + 1)
            && next.is_ascii_digit()
        {
            let name = &input[..i];
            let version = &input[i + 1..];
            return (name, Some(version));
        }
    }

    (input, None)
}

fn extract_maintainer(path: &Path) -> io::Result<Option<String>> {
    let s = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(s.as_str());
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut in_maintainer = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"maintainer" => {
                in_maintainer = true;
            }

            Ok(Event::Text(e)) if in_maintainer => {
                match e.xml_content() {
                    Ok(s) => {
                        return Ok(Some(s.into()));
                    }
                    Err(_) => return Ok(None),
                };
            }

            Ok(Event::End(e)) if e.name().as_ref() == b"maintainer" => {
                in_maintainer = false;
            }

            Ok(Event::Eof) | Err(_) => break,

            _ => {}
        }

        buf.clear();
    }

    Ok(None)
}

impl Portage {
    pub fn new() -> Self {
        Self {
            installed_packages: vec![],
            world_packages: vec![],
        }
    }

    pub fn load_world_packages(&mut self) -> io::Result<()> {
        let world = fs::read_to_string("/var/lib/portage/world")?;
        self.world_packages = world
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect();
        Ok(())
    }

    pub fn load_installed_packages(&mut self) -> io::Result<()> {
        let categories = fs::read_dir("/var/db/pkg")?;

        for category in categories {
            let Ok(category) = category else {
                continue;
            };

            let cat_name = category.file_name();
            let pkgs = fs::read_dir(category.path())?;
            for pkg in pkgs {
                let Ok(pkg) = pkg else {
                    continue;
                };

                let pkg_name = pkg.file_name();
                let Some(pkg_name) = pkg_name.to_str() else {
                    continue;
                };

                let (pkg_name, pkg_version) = split_pkg(pkg_name);

                let use_flags_file = fs::read_to_string(pkg.path().join("USE"))?;
                let use_flags: HashSet<&str> = use_flags_file.trim().split(' ').collect();

                let iuse_flags_file = fs::read_to_string(pkg.path().join("IUSE"))?;
                let iuse_flags: Vec<&str> = iuse_flags_file.trim().split(' ').collect();

                let use_flags: HashSet<UseFlag> = iuse_flags
                    .iter()
                    .filter(|x| !x.trim().is_empty())
                    .map(|&original_iuse| {
                        let iuse = original_iuse.strip_prefix("+").unwrap_or(original_iuse);
                        let is_default = iuse != original_iuse;

                        let is_active = use_flags.contains(&iuse);

                        UseFlag::new(iuse.into(), is_active, is_default)
                    })
                    .collect();

                let repository = fs::read_to_string(pkg.path().join("repository"))?
                    .trim()
                    .to_string();
                let homepage: Option<Vec<String>> =
                    match fs::read_to_string(pkg.path().join("HOMEPAGE")) {
                        Ok(homepage) => Some(
                            homepage
                                .split_whitespace()
                                .map(std::string::ToString::to_string)
                                .collect(),
                        ),
                        Err(_) => None,
                    };

                let license: Option<String> = match fs::read_to_string(pkg.path().join("LICENSE")) {
                    Ok(license) => Some(license.trim().to_string()),
                    Err(_) => None,
                };

                let description: Option<String> =
                    match fs::read_to_string(pkg.path().join("DESCRIPTION")) {
                        Ok(description) => Some(description.trim().to_string()),
                        Err(_) => None,
                    };

                let size: usize = match fs::read_to_string(pkg.path().join("SIZE")) {
                    Ok(size) => size.trim().parse().unwrap_or(0),
                    Err(_) => 0,
                };

                let repo_path = PathBuf::from("/var/db/repos/")
                    .join(&repository)
                    .join(cat_name.to_str().unwrap_or_default())
                    .join(pkg_name)
                    .join("metadata.xml");

                let maintainer = extract_maintainer(&repo_path).unwrap_or(None);

                self.installed_packages.push(Package {
                    name: format!("{}/{}", cat_name.to_str().unwrap_or_default(), pkg_name),
                    use_flags,
                    version: pkg_version.unwrap_or_default().into(),
                    repository,
                    maintainer,
                    description,
                    homepage,
                    license,
                    size,
                });
            }
        }

        Ok(())
    }
}
