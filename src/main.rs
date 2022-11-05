#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Install {
        #[clap(value_parser)]
        product: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/KatanPanel/cli/main/scripts/install_ui";
    println!("Fetching contents from {:?}...", url);
    let contents = reqwest::get(url)
        .await?
        .text()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    println!("Contents fetched");

    let options = run_script::ScriptOptions::new();
    let (_, output, _) = run_script::run_script!(contents, &options).unwrap();

    println!("{}", output);

    Ok(())
}
