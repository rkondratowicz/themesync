use anyhow::Result;
use clap::Parser;

mod adapter;
mod adapters;
mod cli;
mod commands;
mod config;

use cli::{AppCommands, Cli, Commands, ThemeCommands};
use config::Config;

async fn run_cli() -> Result<()> {
    let cli = Cli::parse();
    let config_path = Config::get_config_path();
    let mut config = Config::load_from_file(&config_path)?;

    match &cli.command {
        Some(Commands::Set { theme }) => {
            commands::set_theme(&mut config, &config_path, theme).await?;
        }
        Some(Commands::Toggle) => {
            commands::toggle_theme(&mut config, &config_path).await?;
        }
        Some(Commands::Status) => {
            commands::show_status(&config).await?;
        }
        Some(Commands::Themes(ThemeCommands::List)) => {
            commands::list_themes(&config)?;
        }
        Some(Commands::Apps(AppCommands::List)) => {
            commands::list_apps(&config).await?;
        }
        None => {
            println!("Use --help for usage information");
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_cli().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
