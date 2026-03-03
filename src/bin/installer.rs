use include_dir::{Dir, include_dir};
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;
use std::process::Command;
use vigil::config::{CLI_BIN_NAME, DAEMON_BIN_NAME, LOGGER_BIN_NAME, REPO, get_project_dirs};

static ASSETS_DIR: Dir = include_dir!("assets");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().user_agent("vigil-installer").build()?;

    println!("Identifying OS and Architecture...");
    let (os, arch) = (env::consts::OS, env::consts::ARCH);
    println!("OS: {}, Arch: {}", os, arch);

    let proj_dirs = get_project_dirs().ok_or("Could not determine project directories")?;
    let bin_dir = proj_dirs.data_local_dir().join("bin");
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(&bin_dir)?;
    fs::create_dir_all(&config_dir)?;

    println!("Provisioning configuration...");
    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        let config_template = ASSETS_DIR.get_file("config.toml").unwrap().contents();
        fs::write(&config_path, config_template)?;
        println!("Created default config at: {:?}", config_path);
    }

    println!("Fetching latest release information from GitHub...");
    let release_url = format!("https://api.github.com/repos/{}/releases/latest", REPO);
    let release_json: serde_json::Value = client.get(release_url).send()?.json()?;

    let tag = release_json["tag_name"]
        .as_str()
        .ok_or("No release tag found")?;
    println!("Latest version: {}", tag);

    // Identify assets to download
    let daemon_asset_name = match (os, arch) {
        ("linux", "x86_64") => format!("{}-linux-x86_64", DAEMON_BIN_NAME),
        ("macos", "x86_64") => format!("{}-macos-x64", DAEMON_BIN_NAME),
        ("macos", "aarch64") => format!("{}-macos-arm64", DAEMON_BIN_NAME),
        ("windows", "x86_64") => format!("{}-windows-x64.exe", DAEMON_BIN_NAME),
        _ => return Err(format!("Unsupported platform: {}-{}", os, arch).into()),
    };

    let cli_asset_name = match (os, arch) {
        ("linux", "x86_64") => format!("{}-linux-x86_64", CLI_BIN_NAME),
        ("macos", "x86_64") => format!("{}-macos-x64", CLI_BIN_NAME),
        ("macos", "aarch64") => format!("{}-macos-arm64", CLI_BIN_NAME),
        ("windows", "x86_64") => format!("{}-windows-x64.exe", CLI_BIN_NAME),
        _ => return Err(format!("Unsupported platform: {}-{}", os, arch).into()),
    };

    let logger_asset_name = match (os, arch) {
        ("linux", "x86_64") => format!("{}-x86_64-unknown-linux-gnu", LOGGER_BIN_NAME),
        ("macos", "x86_64") => format!("{}-x86_64-apple-darwin", LOGGER_BIN_NAME),
        ("macos", "aarch64") => format!("{}-aarch64-apple-darwin", LOGGER_BIN_NAME),
        ("windows", "x86_64") => format!("{}-windows-x64.exe", LOGGER_BIN_NAME),
        _ => return Err(format!("Unsupported platform: {}-{}", os, arch).into()),
    };

    download_asset(
        &client,
        &release_json,
        &daemon_asset_name,
        &bin_dir.join(if os == "windows" {
            format!("{}.exe", DAEMON_BIN_NAME)
        } else {
            DAEMON_BIN_NAME.to_string()
        }),
    )?;
    download_asset(
        &client,
        &release_json,
        &cli_asset_name,
        &bin_dir.join(if os == "windows" {
            format!("{}.exe", CLI_BIN_NAME)
        } else {
            CLI_BIN_NAME.to_string()
        }),
    )?;
    download_asset(
        &client,
        &release_json,
        &logger_asset_name,
        &bin_dir.join(if os == "windows" {
            format!("{}.exe", LOGGER_BIN_NAME)
        } else {
            LOGGER_BIN_NAME.to_string()
        }),
    )?;

    println!("Setting up auto-start...");
    setup_autostart(os, &bin_dir)?;

    println!("Installation complete! Vigil will now start automatically on boot.");
    Ok(())
}

fn download_asset(
    client: &Client,
    release_json: &serde_json::Value,
    asset_name: &str,
    dest: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let assets = release_json["assets"].as_array().ok_or("No assets found")?;
    let asset = assets
        .iter()
        .find(|a| a["name"].as_str() == Some(asset_name))
        .ok_or(format!("Asset {} not found in release", asset_name))?;

    let download_url = asset["browser_download_url"]
        .as_str()
        .ok_or("No download URL")?;
    println!("Downloading {}...", asset_name);

    let mut response = client.get(download_url).send()?;
    let mut file = fs::File::create(dest)?;
    io::copy(&mut response, &mut file)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(dest)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(dest, perms)?;
    }

    Ok(())
}

fn setup_autostart(os: &str, bin_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let vigil_path = bin_dir.join(if os == "windows" {
        format!("{}.exe", DAEMON_BIN_NAME)
    } else {
        DAEMON_BIN_NAME.to_string()
    });

    match os {
        "linux" => {
            let service_template = ASSETS_DIR
                .get_file("templates/vigil.service")
                .unwrap()
                .contents_utf8()
                .unwrap();
            let service_content =
                service_template.replace("{{VIGIL_PATH}}", vigil_path.to_str().unwrap());

            let service_path = dirs::config_dir()
                .unwrap()
                .join("systemd/user/vigil.service");
            fs::create_dir_all(service_path.parent().unwrap())?;
            fs::write(&service_path, service_content)?;
            Command::new("systemctl")
                .args(&["--user", "enable", "--now", "vigil.service"])
                .status()?;
        }
        "macos" => {
            let plist_template = ASSETS_DIR
                .get_file("templates/com.44103.vigil.plist")
                .unwrap()
                .contents_utf8()
                .unwrap();
            let plist_content =
                plist_template.replace("{{VIGIL_PATH}}", vigil_path.to_str().unwrap());

            let plist_path = dirs::home_dir()
                .unwrap()
                .join("Library/LaunchAgents/com.44103.vigil.plist");
            fs::create_dir_all(plist_path.parent().unwrap())?;
            fs::write(&plist_path, plist_content)?;
            Command::new("launchctl")
                .args(&["load", plist_path.to_str().unwrap()])
                .status()?;
        }
        "windows" => {
            let startup_dir = dirs::config_dir()
                .unwrap()
                .join("Microsoft\\Windows\\Start Menu\\Programs\\Startup");
            let target_path = startup_dir.join(format!("{}.exe", DAEMON_BIN_NAME));
            fs::copy(&vigil_path, target_path)?;
        }
        _ => println!("Auto-start setup skipped for this OS."),
    }
    Ok(())
}
