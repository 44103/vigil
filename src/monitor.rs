use active_win_pos_rs::get_active_window;
use sysinfo::{Pid, System};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use crate::config::{load_config, resolve_data_dir};

pub async fn run() {
    let mut system = System::new_all();
    let config = load_config();
    
    let interval = config.monitor_interval_secs.unwrap_or(60);
    let data_dir = resolve_data_dir(&config);

    if let Err(e) = fs::create_dir_all(&data_dir) {
        eprintln!("Error creating data directory {:?}: {e}", data_dir);
    }

    loop {
        system.refresh_processes();

        if let Err(e) = log_active_window(&system, &data_dir).await {
            eprintln!("Error logging active window: {e}");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
    }
}

async fn log_active_window(system: &System, data_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let file_path = data_dir.join(format!("{date}.csv"));

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)?;

    if file.metadata()?.len() == 0 {
        writeln!(file, "DateTime,ProcessName,WindowTitle")?;
    }

    match get_active_window() {
        Ok(active_window) => {
            // u64 -> u32 -> Pid -> Process -> Name
            let process_name = active_window.process_id
                .try_into()
                .ok()
                .map(Pid::from_u32)
                .and_then(|pid| system.process(pid))
                .map(|p| p.name().to_string())
                .unwrap_or_else(|| format!("Unknown (PID: {})", active_window.process_id));

            writeln!(file, "{timestamp},{process_name},{}", active_window.title)?;
            println!("Logged: {timestamp} - {process_name} - {}", active_window.title);
        }
        Err(e) => {
            eprintln!("Could not get active window: {e:?}");
            writeln!(file, "{timestamp},Error,{e:?}")?;
        }
    }

    Ok(())
}
