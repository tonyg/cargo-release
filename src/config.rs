use std::path::{Path, PathBuf};

use clap::arg_enum;
use serde::{Deserialize, Serialize};

use crate::error::FatalError;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub exclude_paths: Option<Vec<String>>,
    pub sign_commit: Option<bool>,
    pub sign_tag: Option<bool>,
    pub push_remote: Option<String>,
    pub registry: Option<String>,
    pub disable_release: Option<bool>,
    pub disable_publish: Option<bool>,
    pub no_verify: Option<bool>,
    pub disable_push: Option<bool>,
    pub push_options: Option<Vec<String>>,
    pub dev_version_ext: Option<String>,
    pub no_dev_version: Option<bool>,
    pub consolidate_commits: Option<bool>,
    pub consolidate_pushes: Option<bool>,
    pub pre_release_commit_message: Option<String>,
    pub post_release_commit_message: Option<String>,
    pub pre_release_replacements: Option<Vec<Replace>>,
    pub post_release_replacements: Option<Vec<Replace>>,
    pub pre_release_hook: Option<Command>,
    pub tag_message: Option<String>,
    pub tag_prefix: Option<String>,
    pub tag_name: Option<String>,
    pub disable_tag: Option<bool>,
    pub enable_features: Option<Vec<String>>,
    pub enable_all_features: Option<bool>,
    pub dependent_version: Option<DependentVersion>,
}

impl Config {
    pub fn update(&mut self, source: &Config) {
        if let Some(exclude_paths) = source.exclude_paths.as_deref() {
            self.exclude_paths = Some(exclude_paths.to_vec());
        }
        if let Some(sign_commit) = source.sign_commit {
            self.sign_commit = Some(sign_commit);
        }
        if let Some(sign_tag) = source.sign_tag {
            self.sign_tag = Some(sign_tag);
        }
        if let Some(push_remote) = source.push_remote.as_deref() {
            self.push_remote = Some(push_remote.to_owned());
        }
        if let Some(registry) = source.registry.as_deref() {
            self.registry = Some(registry.to_owned());
        }
        if let Some(disable_release) = source.disable_release {
            self.disable_release = Some(disable_release);
        }
        if let Some(disable_publish) = source.disable_publish {
            self.disable_publish = Some(disable_publish);
        }
        if let Some(no_verify) = source.no_verify {
            self.no_verify = Some(no_verify);
        }
        if let Some(disable_push) = source.disable_push {
            self.disable_push = Some(disable_push);
        }
        if let Some(push_options) = source.push_options.as_deref() {
            self.push_options = Some(push_options.to_owned());
        }
        if let Some(dev_version_ext) = source.dev_version_ext.as_deref() {
            self.dev_version_ext = Some(dev_version_ext.to_owned());
        }
        if let Some(no_dev_version) = source.no_dev_version {
            self.no_dev_version = Some(no_dev_version);
        }
        if let Some(consolidate_commits) = source.consolidate_commits {
            self.consolidate_commits = Some(consolidate_commits);
        }
        if let Some(consolidate_pushes) = source.consolidate_pushes {
            self.consolidate_pushes = Some(consolidate_pushes);
        }
        if let Some(pre_release_commit_message) = source.pre_release_commit_message.as_deref() {
            self.pre_release_commit_message = Some(pre_release_commit_message.to_owned());
        }
        if let Some(post_release_commit_message) = source.post_release_commit_message.as_deref() {
            self.post_release_commit_message = Some(post_release_commit_message.to_owned());
        }
        if let Some(pre_release_replacements) = source.pre_release_replacements.as_deref() {
            self.pre_release_replacements = Some(pre_release_replacements.to_owned());
        }
        if let Some(post_release_replacements) = source.post_release_replacements.as_deref() {
            self.post_release_replacements = Some(post_release_replacements.to_owned());
        }
        if let Some(pre_release_hook) = source.pre_release_hook.as_ref() {
            self.pre_release_hook = Some(pre_release_hook.to_owned());
        }
        if let Some(tag_message) = source.tag_message.as_deref() {
            self.tag_message = Some(tag_message.to_owned());
        }
        if let Some(tag_prefix) = source.tag_prefix.as_deref() {
            self.tag_prefix = Some(tag_prefix.to_owned());
        }
        if let Some(tag_name) = source.tag_name.as_deref() {
            self.tag_name = Some(tag_name.to_owned());
        }
        if let Some(disable_tag) = source.disable_tag {
            self.disable_tag = Some(disable_tag);
        }
        if let Some(enable_features) = source.enable_features.as_deref() {
            self.enable_features = Some(enable_features.to_owned());
        }
        if let Some(enable_all_features) = source.enable_all_features {
            self.enable_all_features = Some(enable_all_features);
        }
        if let Some(dependent_version) = source.dependent_version {
            self.dependent_version = Some(dependent_version);
        }
    }

    pub fn exclude_paths(&self) -> Option<&[String]> {
        self.exclude_paths.as_ref().map(|v| v.as_ref())
    }

    pub fn sign_commit(&self) -> bool {
        self.sign_commit.unwrap_or(false)
    }

    pub fn sign_tag(&self) -> bool {
        self.sign_tag.unwrap_or(false)
    }

    pub fn push_remote(&self) -> &str {
        self.push_remote.as_deref().unwrap_or("origin")
    }

    pub fn registry(&self) -> Option<&str> {
        self.registry.as_deref()
    }

    pub fn disable_release(&self) -> bool {
        self.disable_release.unwrap_or(false)
    }

    pub fn disable_publish(&self) -> bool {
        self.disable_publish.unwrap_or(false)
    }

    pub fn no_verify(&self) -> bool {
        self.no_verify.unwrap_or(false)
    }

    pub fn disable_push(&self) -> bool {
        self.disable_push.unwrap_or(false)
    }

    pub fn push_options(&self) -> &[String] {
        self.push_options
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&[])
    }

    pub fn dev_version_ext(&self) -> &str {
        self.dev_version_ext.as_deref().unwrap_or("alpha.0")
    }

    pub fn no_dev_version(&self) -> bool {
        self.no_dev_version.unwrap_or(false)
    }

    pub fn consolidate_commits(&self) -> bool {
        self.consolidate_commits.unwrap_or(false)
    }

    pub fn consolidate_pushes(&self) -> bool {
        self.consolidate_pushes.unwrap_or(false)
    }

    pub fn pre_release_commit_message(&self) -> &str {
        self.pre_release_commit_message
            .as_deref()
            .unwrap_or("(cargo-release) version {{version}}")
    }

    pub fn post_release_commit_message(&self) -> &str {
        self.post_release_commit_message
            .as_deref()
            .unwrap_or("(cargo-release) start next development iteration {{next_version}}")
    }

    pub fn pre_release_replacements(&self) -> &[Replace] {
        self.pre_release_replacements
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&[])
    }

    pub fn post_release_replacements(&self) -> &[Replace] {
        self.post_release_replacements
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&[])
    }

    pub fn pre_release_hook(&self) -> Option<&Command> {
        self.pre_release_hook.as_ref()
    }

    pub fn tag_message(&self) -> &str {
        self.tag_message
            .as_deref()
            .unwrap_or("(cargo-release) {{crate_name}} version {{version}}")
    }

    pub fn tag_prefix(&self, is_root: bool) -> &str {
        // crate_name as default tag prefix for multi-crate project
        self.tag_prefix
            .as_deref()
            .unwrap_or_else(|| if !is_root { "{{crate_name}}-" } else { "" })
    }

    pub fn tag_name(&self) -> &str {
        self.tag_name.as_deref().unwrap_or("{{prefix}}v{{version}}")
    }

    pub fn disable_tag(&self) -> bool {
        self.disable_tag.unwrap_or(false)
    }

    pub fn enable_features(&self) -> &[String] {
        self.enable_features
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&[])
    }

    pub fn enable_all_features(&self) -> bool {
        self.enable_all_features.unwrap_or(false)
    }

    pub fn dependent_version(&self) -> DependentVersion {
        self.dependent_version.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Replace {
    pub file: PathBuf,
    pub search: String,
    pub replace: String,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub exactly: Option<usize>,
    #[serde(default)]
    pub prerelease: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Line(String),
    Args(Vec<String>),
}

impl Command {
    pub fn args(&self) -> Vec<&str> {
        match self {
            Command::Line(ref s) => vec![s.as_str()],
            Command::Args(ref a) => a.iter().map(|s| s.as_str()).collect(),
        }
    }
}

arg_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum DependentVersion {
        Upgrade,
        Fix,
        Error,
        Warn,
        Ignore,
    }
}

impl Default for DependentVersion {
    fn default() -> Self {
        DependentVersion::Fix
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoManifest {
    workspace: Option<CargoWorkspace>,
    package: Option<CargoPackage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoWorkspace {
    metadata: Option<CargoMetadata>,
}

impl CargoWorkspace {
    fn into_config(self) -> Option<Config> {
        self.metadata?.release
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoPackage {
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    metadata: Option<CargoMetadata>,
}

impl CargoPackage {
    fn into_config(self) -> Option<Config> {
        if self.include.is_none() && self.exclude.is_none() && self.metadata.is_none() {
            return None;
        }
        let CargoPackage {
            include,
            exclude,
            metadata,
        } = self;
        let mut config = metadata.and_then(|m| m.release).unwrap_or_default();
        if config.exclude_paths.is_none() {
            if let Some(_include) = include {
                log::trace!("Ignoring `include` from Cargo.toml");
            } else if let Some(exclude) = exclude {
                config.exclude_paths = Some(exclude);
            }
        }
        Some(config)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoMetadata {
    release: Option<Config>,
}

fn get_pkg_config_from_manifest(manifest_path: &Path) -> Result<Option<Config>, FatalError> {
    if manifest_path.exists() {
        let m = std::fs::read_to_string(manifest_path).map_err(FatalError::from)?;
        let c: CargoManifest = toml::from_str(&m).map_err(FatalError::from)?;

        Ok(c.package.and_then(|p| p.into_config()))
    } else {
        Ok(None)
    }
}

fn get_ws_config_from_manifest(manifest_path: &Path) -> Result<Option<Config>, FatalError> {
    if manifest_path.exists() {
        let m = std::fs::read_to_string(manifest_path).map_err(FatalError::from)?;
        let c: CargoManifest = toml::from_str(&m).map_err(FatalError::from)?;

        Ok(c.workspace.and_then(|p| p.into_config()))
    } else {
        Ok(None)
    }
}

fn get_config_from_file(file_path: &Path) -> Result<Option<Config>, FatalError> {
    if file_path.exists() {
        let c = std::fs::read_to_string(file_path).map_err(FatalError::from)?;
        let config = toml::from_str(&c).map_err(FatalError::from)?;
        Ok(Some(config))
    } else {
        Ok(None)
    }
}

pub fn resolve_custom_config(file_path: &Path) -> Result<Option<Config>, FatalError> {
    get_config_from_file(file_path)
}

/// Try to resolve workspace configuration source.
///
/// This tries the following sources in order, merging the results:
/// 1. $HOME/.release.toml
/// 2. $(workspace)/release.toml
pub fn resolve_workspace_config(workspace_root: &Path) -> Result<Config, FatalError> {
    let mut config = Config::default();

    // User-local configuration from home directory.
    let home_dir = dirs_next::home_dir();
    if let Some(mut home) = home_dir {
        home.push(".release.toml");
        if let Some(cfg) = get_config_from_file(&home)? {
            config.update(&cfg);
        }
    };

    let default_config = workspace_root.join("release.toml");
    let current_dir_config = get_config_from_file(&default_config)?;
    if let Some(cfg) = current_dir_config {
        config.update(&cfg);
    };

    // Crate manifest.
    let manifest_path = workspace_root.join("Cargo.toml");
    let current_dir_config = get_ws_config_from_manifest(&manifest_path)?;
    if let Some(cfg) = current_dir_config {
        config.update(&cfg);
    };

    Ok(config)
}

/// Try to resolve configuration source.
///
/// This tries the following sources in order, merging the results:
/// 1. $HOME/.release.toml
/// 2. $(workspace)/release.toml
/// 4. $(crate)/release.toml
/// 3. $(crate)/Cargo.toml `package.metadata.release`
///
/// `$(crate)/Cargo.toml` is a way to differentiate configuration for the root crate and the
/// workspace.
pub fn resolve_config(workspace_root: &Path, manifest_path: &Path) -> Result<Config, FatalError> {
    let mut config = Config::default();

    // User-local configuration from home directory.
    let home_dir = dirs_next::home_dir();
    if let Some(mut home) = home_dir {
        home.push(".release.toml");
        if let Some(cfg) = get_config_from_file(&home)? {
            config.update(&cfg);
        }
    };

    let crate_root = manifest_path.parent().unwrap_or_else(|| Path::new("."));

    if crate_root != workspace_root {
        let default_config = workspace_root.join("release.toml");
        let current_dir_config = get_config_from_file(&default_config)?;
        if let Some(cfg) = current_dir_config {
            config.update(&cfg);
        };
    }

    // Project release file.
    let default_config = crate_root.join("release.toml");
    let current_dir_config = get_config_from_file(&default_config)?;
    if let Some(cfg) = current_dir_config {
        config.update(&cfg);
    };

    // Crate manifest.
    let current_dir_config = get_pkg_config_from_manifest(manifest_path)?;
    if let Some(cfg) = current_dir_config {
        config.update(&cfg);
    };

    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;

    mod resolve_config {
        use super::*;

        #[test]
        fn doesnt_panic() {
            let release_config = resolve_config(Path::new("."), Path::new("Cargo.toml")).unwrap();
            assert!(release_config.sign_commit());
        }
    }
}
