use reqwest;
use serde::Deserialize;
use webbrowser;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[tauri::command]
pub fn open_link(url: &str) {
    if let Err(e) = webbrowser::open(&url) {
        eprintln!("Failed to open link: {}", e);
    }
}

pub fn get_latest_version(current_version: &str) -> String {
    let url = "https://api.github.com/repos/initialencounter/rainwarm/releases/latest";
    let client = reqwest::blocking::Client::new();

    let resp = match client
        .get(url)
        .header(reqwest::header::USER_AGENT, "rust-app")
        .send()
    {
        Ok(response) => response,
        Err(_) => return current_version.to_string(),
    };

    if !resp.status().is_success() {
        return current_version.to_string();
    }

    let release = match resp.json::<Release>() {
        Ok(release) => release,
        Err(_) => return current_version.to_string(),
    };

    release.tag_name
}
