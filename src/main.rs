mod cli;
mod config;
mod error;
mod export;
mod flow;
mod okta;

#[tokio::main]
async fn main() {
    if let Err(err) = cli::run().await {
        eprintln!("error: {err}");

        std::process::exit(1);
    }
}
