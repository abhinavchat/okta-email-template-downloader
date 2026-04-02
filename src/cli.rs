use clap::Parser;

use crate::config::{AppConfig, ResolvedConfig};
use crate::error::AppResult;
use crate::flow;
use crate::okta::client::OktaClient;

#[derive(Debug, Clone, Parser)]
#[command(name = "okta-template-downloader")]
#[command(about = "Download Okta email templates as HTML and subject files")]
pub struct Cli {
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    #[arg(long)]
    pub domain: Option<String>,

    #[arg(long)]
    pub token: Option<String>,

    #[arg(long)]
    pub output: Option<std::path::PathBuf>,

    #[arg(long)]
    pub brand: Option<String>,

    #[arg(long)]
    pub template: Option<String>,

    #[arg(long)]
    pub all: bool,

    #[arg(long)]
    pub non_interactive: bool,

    #[arg(long)]
    pub overwrite: bool,

    #[arg(long)]
    pub verbose: bool,
}

pub async fn run() -> AppResult<()> {
    let cli = Cli::parse();
    let resolved = AppConfig::load(&cli)?;
    let client = OktaClient::new(&resolved)?;

    let requested = ResolvedConfig {
        brand: cli.brand,
        template: cli.template,
        all: cli.all,
        output: cli.output,
        overwrite: cli.overwrite,
        non_interactive: cli.non_interactive,
        verbose: cli.verbose,
        domain: resolved.domain,
        token: resolved.token,
        output_dir: resolved.output_dir,
        config_path: resolved.config_path,
    };

    flow::run(requested, client).await
}
