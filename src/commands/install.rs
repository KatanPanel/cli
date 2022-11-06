use std::io::Write;
use tokio::io::AsyncWriteExt;

const URL: &str = "https://raw.githubusercontent.com/KatanPanel/cli/main/bin/install_ui";
const PACKAGES: &[&str] = &["web-ui"];

type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

pub async fn install(package: String) -> Result<(), Box<dyn std::error::Error>> {
    if !PACKAGES.contains(&package.as_str()) {
        return Ok(println!("Invalid package: {}", package));
    }

    let client = reqwest::Client::new();
    println!("Downloading package...");

    // download_file(client, URL, package.as_str()).await.unwrap();

    println!("Reading package...");
    let contents = std::fs::read_to_string(package).expect("Failed to read package contents");

    println!("Installing package...");

    let options = run_script::ScriptOptions {
        runner: None,
        working_directory: None,
        input_redirection: run_script::types::IoOptions::Inherit,
        output_redirection: run_script::types::IoOptions::Inherit,
        exit_on_error: true,
        print_commands: false,
        env_vars: None,
    };

    let args = vec![];
    run_script::spawn(contents.as_str(), &args, &options)
        .unwrap()
        .wait_with_output()?;
    Ok(())
}

pub async fn download_file(client: reqwest::Client, url: &str, path: &str) -> Result<(), BoxError> {
    let uri = reqwest::Url::parse(url)?;
    let content_length = {
        let res = client.head(uri.as_str()).send().await?;
        if !res.status().is_success() {
            return Err(format!("Failed to HEAD {} (status: {:?})", uri, res.status()).into());
        }

        let value = res
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|len| len.to_str().ok())
            .and_then(|len| len.parse::<u64>().ok())
            .unwrap_or(0);
        value
    };

    let req = client.get(url);
    let progress_bar = indicatif::ProgressBar::new(content_length);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} - {msg}")?
            .progress_chars("#>-"),
    );
    progress_bar.set_message("Downloading...");

    let mut outfile = tokio::fs::File::create(path).await?;
    let mut download = req.send().await?;

    while let Some(frame) = download.chunk().await? {
        progress_bar.inc(frame.len() as u64);
        outfile.write(&frame).await?;
    }

    progress_bar.finish();
    outfile.flush().await?;
    return Ok(());
}
