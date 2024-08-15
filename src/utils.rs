// src/utils.rs

use regex::Regex;
use std::process::Command;
use tempfile::TempDir;

pub fn is_url(input: &str) -> bool {
    input.starts_with("http://") || input.starts_with("https://")
}

pub fn extract_github_repo_root(url: &str) -> Option<String> {
    let re = Regex::new(r"https://github.com/([^/]+/[^/]+)").unwrap();
    re.captures(url).map(|cap| cap[0].to_string())
}

pub fn download_repo(repo_url: &str) -> Result<TempDir, anyhow::Error> {
    let temp_dir = TempDir::new()?;
    let output = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(temp_dir.path())
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to clone repository"));
    }

    Ok(temp_dir)
}
