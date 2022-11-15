const BASE_URL: &str = "https://raw.githubusercontent.com/KatanPanel/cli/main/bin/install_";
const PACKAGES: &[&str] = &["web-ui", "cp"];

pub async fn install(
    client: &reqwest::Client,
    package: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !PACKAGES.contains(&package.as_str()) {
        return Ok(println!("Invalid package: {}", package));
    }

    let url = format!("{BASE_URL}{package}").replace("-", "-");
    println!("Fetching from {}...", url);

    let contents = download_file(client, &url).await.expect("Failed to download file contents");

    let options = run_script::ScriptOptions {
        runner: None,
        working_directory: None,
        input_redirection: run_script::types::IoOptions::Inherit,
        output_redirection: run_script::types::IoOptions::Inherit,
        exit_on_error: true,
        print_commands: false,
        env_vars: None,
    };

    run_script::spawn(contents.as_str(), &vec![], &options)
        .unwrap()
        .wait_with_output()?;
    Ok(())
}

pub async fn download_file(
    client: &reqwest::Client,
    url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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

    let mut contents = "".to_owned();
    let mut download = req.send().await?;

    while let Some(frame) = download.chunk().await? {
        progress_bar.inc(frame.len() as u64);
        contents.push_str(std::str::from_utf8(&frame.to_vec())?);
    }

    progress_bar.finish();
    return Ok(contents);
}
