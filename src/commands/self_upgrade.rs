use anyhow::{Context, Result};
use serde::Deserialize;
use std::io::Write;

pub fn run() -> Result<()> {
    let latest =
        get_latest_release().with_context(|| "i failed to fetch latest release from GitHub")?;

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let latest_version = latest.tag_name.trim_start_matches('v');

    if !is_newer(latest_version, VERSION)? {
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
    let temp = current.with_extension("tmp");
    let installed = (|| -> Result<()> {
        download_to(&asset.browser_download_url, &temp)?;

        // Set executable permissions (Linux/macOS)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&temp, std::fs::Permissions::from_mode(0o755))?;
        }

        // Atomic replace
        std::fs::rename(&temp, &current)?;
        Ok(())
    })();
    if let Err(e) = installed {
        let _ = std::fs::remove_file(&temp);
        return Err(e).context("i couldn't install the new version - nothing was changed");
    }

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
        .send()?
        .error_for_status()?;

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

fn download_to(url: &str, path: &std::path::Path) -> anyhow::Result<()> {
    let bytes = download_asset(url)?;

    let mut file = std::fs::File::create(path)?;
    file.write_all(&bytes)?;

    Ok(())
}

/// Parse an `x.y.z` version (ignoring any `-pre`/`+build` suffix).
fn parse_version(version: &str) -> anyhow::Result<(u64, u64, u64)> {
    let core = version.split(['-', '+']).next().unwrap_or(version);
    let mut parts = core.split('.').map(str::parse::<u64>);
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(Ok(major)), Some(Ok(minor)), Some(Ok(patch)), None) => Ok((major, minor, patch)),
        _ => anyhow::bail!("i can't make sense of version '{}'", version),
    }
}

fn is_newer(latest: &str, current: &str) -> anyhow::Result<bool> {
    Ok(parse_version(latest)? > parse_version(current)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_versions_numerically() {
        assert!(is_newer("0.10.0", "0.9.2").unwrap());
        assert!(is_newer("1.0.0", "0.99.99").unwrap());
        assert!(!is_newer("0.9.2", "0.9.2").unwrap());
        assert!(!is_newer("0.9.1", "0.9.2").unwrap());
        assert!(!is_newer("0.9.2", "0.10.0").unwrap());
        assert!(is_newer("0.9.3-rc.1", "0.9.2").unwrap());
    }

    #[test]
    fn rejects_garbage_versions() {
        assert!(parse_version("latest").is_err());
        assert!(parse_version("1.2").is_err());
        assert!(parse_version("1.2.3.4").is_err());
    }
}
