use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FileConfig {
    pub okta_domain: Option<String>,
    pub api_token: Option<String>,
    pub output_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub domain: String,
    pub token: String,
    pub output_dir: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub brand: Option<String>,
    pub template: Option<String>,
    pub all: bool,
    pub output: Option<PathBuf>,
    pub overwrite: bool,
    pub non_interactive: bool,
    pub verbose: bool,
}
