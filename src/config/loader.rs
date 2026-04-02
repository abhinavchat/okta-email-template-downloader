use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::cli::Cli;
use crate::config::model::{FileConfig, ResolvedConfig};
use crate::error::{AppError, AppResult};

pub struct AppConfig;

impl AppConfig {
    pub fn load(cli: &Cli) -> AppResult<ResolvedConfig> {
        let config_path = if let Some(path) = cli.config.clone() {
            Some(path)
        } else {
            discover_config_path()
        };

        let file_config = if let Some(path) = config_path.as_ref() {
            read_file_config(path)?
        } else {
            FileConfig::default()
        };

        let domain = cli
            .domain
            .clone()
            .or_else(|| env::var("OKTA_DOMAIN").ok())
            .or(file_config.okta_domain)
            .ok_or_else(|| AppError::Config {
                message: missing_config_help("OKTA_DOMAIN", "okta_domain"),
            })?;

        let token = cli
            .token
            .clone()
            .or_else(|| env::var("OKTA_API_TOKEN").ok())
            .or(file_config.api_token)
            .ok_or_else(|| AppError::Config {
                message: missing_config_help("OKTA_API_TOKEN", "api_token"),
            })?;

        let output_dir = cli
            .output
            .clone()
            .or_else(|| env::var_os("OKTA_OUTPUT_DIR").map(PathBuf::from))
            .or(file_config.output_dir);

        Ok(ResolvedConfig {
            domain,
            token,
            output_dir,
            config_path,
            brand: None,
            template: None,
            all: false,
            output: None,
            overwrite: false,
            non_interactive: false,
            verbose: cli.verbose,
        })
    }
}

fn read_file_config(path: &Path) -> AppResult<FileConfig> {
    let contents = fs::read_to_string(path).map_err(|source| AppError::ConfigRead {
        path: path.to_path_buf(),
        source,
    })?;

    toml::from_str(&contents).map_err(|source| AppError::ConfigParse {
        path: path.to_path_buf(),
        source,
    })
}

fn discover_config_path() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    let local_hidden = cwd.join(".okta-template-downloader.toml");
    if local_hidden.is_file() {
        return Some(local_hidden);
    }

    let local = cwd.join("okta-template-downloader.toml");
    if local.is_file() {
        return Some(local);
    }

    let project_dirs = ProjectDirs::from("", "", "okta-template-downloader")?;
    let config_dir_path = project_dirs.config_dir().join("config.toml");
    if config_dir_path.is_file() {
        Some(config_dir_path)
    } else {
        None
    }
}

fn missing_config_help(env_name: &str, toml_key: &str) -> String {
    format!(
        "set {env_name} or add '{toml_key}' to .okta-template-downloader.toml, okta-template-downloader.toml, or your user config file"
    )
}

#[cfg(test)]
mod tests {
    use super::missing_config_help;

    #[test]
    fn missing_config_message_mentions_env_and_toml() {
        let msg = missing_config_help("OKTA_DOMAIN", "okta_domain");
        assert!(msg.contains("OKTA_DOMAIN"));
        assert!(msg.contains("okta_domain"));
    }
}
