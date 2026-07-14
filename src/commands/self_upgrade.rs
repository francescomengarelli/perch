use anyhow::{Context, Result};
use serde::Deserialize;
use std::io::Write;

pub fn run() -> Result<()> {
    let latest =
        get_latest_release().with_context(|| "i failed to fetch latest release from GitHub")?;

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let latest_version = latest.tag_name.trim_start_matches('v');

    if latest_version == VERSION {
        println!("i am already up to date ({})", VERSION);
        return Ok(());
    }

    let target = current_target()?;
    let asset = latest
        .assets
        .iter()
        .find(|a| a.name.contains(target))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "i haven't found the right asset for your platform: {}",
                target
            )
        })?;

    eprintln!("i found the asset for {}. downloading it...", target);
    let current = std::env::current_exe()?;
    let temp = download_to_temp(&asset.browser_download_url, &current)?;

    // Set executable permissions (Linux/macOS)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&temp, std::fs::Permissions::from_mode(0o755))?;
    }

    // Atomic replace
    std::fs::rename(&temp, &current)?;

    println!("upgraded to {}", latest.tag_name);

    Ok(())
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn get_latest_release() -> anyhow::Result<Release> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("perch-cli") // GitHub API requires a User-Agent
        .build()?;

    let resp = client
        .get("https://api.github.com/repos/francescomengarelli/perch/releases/latest")
        .send()?;

    let latest = resp.json::<Release>()?;

    Ok(latest)
}

fn current_target() -> anyhow::Result<&'static str> {
    match (std::env::consts::ARCH, std::env::consts::OS) {
        ("x86_64", "macos") => Ok("x86_64-apple-darwin"),
        ("aarch64", "macos") => Ok("aarch64-apple-darwin"),
        ("x86_64", "linux") => Ok("x86_64-unknown-linux-musl"),
        ("aarch64", "linux") => Ok("aarch64-unknown-linux-gnu"),
        (arch, os) => anyhow::bail!("Unsupported platform: {}-{}", arch, os),
    }
}

fn download_asset(url: &str) -> anyhow::Result<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("perch-cli")
        .build()?;

    let bytes = client
        .get(url)
        .send()?
        .error_for_status()?
        .bytes()?
        .to_vec();

    Ok(bytes)
}

fn download_to_temp(
    url: &str,
    current_binary: &std::path::Path,
) -> anyhow::Result<std::path::PathBuf> {
    let bytes = download_asset(url)?;

    let temp_path = current_binary.with_extension("tmp");

    let mut file = std::fs::File::create(&temp_path)?;
    file.write_all(&bytes)?;

    Ok(temp_path)
}
