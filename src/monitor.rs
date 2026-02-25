use active_win_pos_rs::get_active_window;
use sysinfo::{Pid, System};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub async fn run() {
    let mut system = System::new_all();

    loop {
        system.refresh_processes();

        if let Err(e) = log_active_window(&system).await {
            eprintln!("Error logging active window: {e}");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

async fn log_active_window(system: &System) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let file_path = format!("./data/{date}.csv");

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
