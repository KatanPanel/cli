use clap::{Command, Parser, Subcommand};
use std::os::unix::fs::PermissionsExt;
use std::{
    fmt::format,
    fs::{self, File},
    io::Write,
    process::Command as StdCommand,
    sync::Arc,
};

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

    std::fs::set_permissions("./pirocadefoice", std::fs::Permissions::from_mode(0o777)).unwrap();

    let command_result = StdCommand::new(format!("./{}", "pirocadefoice"))
        .output()
        .unwrap();

    dbg!(command_result);
    Ok(())
}
