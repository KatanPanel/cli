use clap::{Command, Parser, Subcommand};
use std::{fmt::format, fs::File, io::Write, process::Command as StdCommand, sync::Arc};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        #[clap(value_parser)]
        product: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        reqwest::get("https://raw.githubusercontent.com/KatanPanel/cli/main/scripts/install_ui")
            .await?
            .text()
            .await?;

    {
        let mut file = File::create("./pirocadefoice")?;
        writeln!(file, "{}", contents)?;
    }

    let command_result = StdCommand::new(format!("./{}", "pirocadefoice"))
        .output()
        .unwrap();

    dbg!(command_result);
    Ok(())
}
