use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use chrono::DateTime;
use color_eyre::eyre::Context;
use quick_xml::{Reader, events::Event};

use crate::gentoo::{
    InstalledPackage, UseFlag,
    package::{Metadata, Package, PackageKey, PackageVersion},
};

fn split_pkg(input: &str) -> (&str, &str) {
    let bytes = input.as_bytes();

    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'-'
            && let Some(next) = bytes.get(i + 1)
            && next.is_ascii_digit()
        {
            let name = &input[..i];
            let version = &input[i + 1..];
            return (name, version);
        }
    }

    (input, "")
}

fn extract_maintainer(path: &Path) -> color_eyre::Result<Option<String>> {
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

fn parse_cache_file(content: &str) -> HashMap<String, String> {
    content
        .lines()
        .filter_map(|line| {
            line.split_once('=')
                .map(|(k, v)| (k.to_string(), v.to_string()))
        })
        .collect()
}

pub fn load_world_packages() -> color_eyre::Result<HashSet<PackageKey>> {
    let world = fs::read_to_string("/var/lib/portage/world")?;
    Ok(world
        .split_whitespace()
        .filter_map(PackageKey::from_atom)
        .collect())
}

pub fn load_available_packages() -> color_eyre::Result<Vec<Package>> {
    let repos = fs::read_dir("/var/db/repos/").wrap_err("failed to read /var/db/repos")?;

    let mut packages: HashMap<PackageKey, Package> = HashMap::new();

    for repo in repos {
        let Ok(repo) = repo else {
            continue;
        };
        let repo_name = repo.file_name();
        let Some(repo_name) = repo_name.to_str() else {
            continue;
        };

        let Ok(categories) = fs::read_dir(repo.path().join("metadata/md5-cache")) else {
            // TODO: Someday parse actual ebuilds instead of md5-cache
            continue;
        };

        for category in categories {
            let Ok(category) = category else {
                continue;
            };

            let Ok(cat_name) = category.file_name().into_string() else {
                continue;
            };

            if cat_name == "Manifest.gz" {
                continue;
            }

            let pkgs = fs::read_dir(category.path())?;

            for pkg in pkgs {
                let Ok(pkg) = pkg else {
                    continue;
                };

                let Ok(pkg_name) = pkg.file_name().into_string() else {
                    continue;
                };

                if pkg_name == "Manifest.gz" {
                    continue;
                }

                let (pkg_name, pkg_version) = split_pkg(&pkg_name);

                let version_file = fs::read_to_string(pkg.path()).wrap_err_with(|| {
                    format!(
                        "failed to read version_file for {cat_name}/{pkg_name}-{pkg_version} in repo {repo_name}"
                    )
                })?;
                let data = parse_cache_file(&version_file);

                let homepages: Vec<String> = data
                    .get("HOMEPAGE")
                    .map(|s| {
                        s.split_whitespace()
                            .map(std::string::ToString::to_string)
                            .collect()
                    })
                    .unwrap_or_default();

                let keywords: Vec<String> = data
                    .get("KEYWORDS")
                    .map(|s| {
                        s.split_whitespace()
                            .map(std::string::ToString::to_string)
                            .collect()
                    })
                    .unwrap_or_default();

                let iuse: Vec<UseFlag> = data
                    .get("IUSE")
                    .map(|s| {
                        s.split_whitespace()
                            .map(|original_iuse| {
                                let iuse = original_iuse.strip_prefix("+").unwrap_or(original_iuse);

                                UseFlag {
                                    name: iuse.to_string(),
                                    default: iuse != original_iuse,
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let repo_path = PathBuf::from("/var/db/repos/")
                    .join(repo_name)
                    .join(&cat_name)
                    .join(pkg_name)
                    .join("metadata.xml");

                let maintainer = extract_maintainer(&repo_path).unwrap_or(None);

                let version = PackageVersion {
                    version: pkg_version.to_string(),
                    metadata: Metadata {
                        maintainer,
                        description: data.get("DESCRIPTION").cloned(),
                        homepage: homepages,
                        license: data.get("LICENSE").cloned(),
                        repository: repo_name.to_string(),
                    },
                    keywords,
                    iuse,
                };

                let pkg_key = PackageKey {
                    category: cat_name.clone(),
                    name: pkg_name.to_string(),
                };

                if let Some(package) = packages.get_mut(&pkg_key) {
                    package.versions.push(version);
                } else {
                    let pkg = Package {
                        atom: pkg_key.clone(),
                        versions: vec![version],
                    };
                    packages.insert(pkg_key, pkg);
                }
            }
        }
    }

    Ok(packages.into_values().collect())
}

pub fn load_installed_packages() -> color_eyre::Result<Vec<InstalledPackage>> {
    let categories = fs::read_dir("/var/db/pkg")?;

    let mut v = vec![];

    for category in categories {
        let Ok(category) = category else {
            continue;
        };

        let Ok(cat_name) = category.file_name().into_string() else {
            continue;
        };

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
            let enabled_use_flags: HashSet<String> = use_flags_file
                .split_whitespace()
                .map(ToString::to_string)
                .collect();

            let iuse_flags_file = fs::read_to_string(pkg.path().join("IUSE"))?;

            let use_flags: Vec<UseFlag> = iuse_flags_file
                .split_whitespace()
                .map(|original_iuse| {
                    let iuse = original_iuse.strip_prefix("+").unwrap_or(original_iuse);

                    UseFlag {
                        name: iuse.to_string(),
                        default: iuse != original_iuse,
                    }
                })
                .collect();

            let repository = fs::read_to_string(pkg.path().join("repository"))?
                .trim()
                .to_string();

            let homepage: Vec<String> = match fs::read_to_string(pkg.path().join("HOMEPAGE")) {
                Ok(homepage) => homepage
                    .split_whitespace()
                    .map(std::string::ToString::to_string)
                    .collect(),
                Err(_) => vec![],
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

            let size: usize = fs::read_to_string(pkg.path().join("SIZE"))?
                .trim()
                .parse()
                .unwrap_or(0);

            let repo_path = PathBuf::from("/var/db/repos/")
                .join(&repository)
                .join(&cat_name)
                .join(pkg_name)
                .join("metadata.xml");

            let build_time = fs::read_to_string(pkg.path().join("BUILD_TIME"))?
                .trim()
                .parse::<i64>()
                .ok()
                .and_then(DateTime::from_timestamp_secs)
                .unwrap_or(DateTime::from_timestamp_nanos(0));

            let maintainer = extract_maintainer(&repo_path).unwrap_or(None);

            let atom = PackageKey {
                category: cat_name.clone(),
                name: pkg_name.to_string(),
            };

            v.push(InstalledPackage {
                atom,
                version: pkg_version.to_string(),
                build_time,
                slot: 0,
                metadata: Metadata {
                    maintainer,
                    description,
                    homepage,
                    license,
                    repository,
                },
                enabled_use_flags,
                iuse: use_flags,
                size,
            });
        }
    }

    Ok(v)
}
