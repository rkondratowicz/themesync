use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "themesync")]
#[command(about = "A multi-application theme switcher")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Switch all configured apps to specified theme")]
    Set {
        #[arg(help = "Theme name to set")]
        theme: String,
    },
    #[command(about = "Toggle between current and previously used theme")]
    Toggle,
    #[command(about = "Show current theme state across all apps")]
    Status,
    #[command(subcommand)]
    Themes(ThemeCommands),
    #[command(subcommand)]
    Apps(AppCommands),
}

#[derive(Subcommand)]
pub enum ThemeCommands {
    #[command(about = "Show available themes")]
    List,
}

#[derive(Subcommand)]
pub enum AppCommands {
    #[command(about = "Show configured applications")]
    List,
}
