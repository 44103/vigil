use active_win_pos_rs::get_active_window;
use sysinfo::{Pid, System};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

#[tokio::main]
async fn main() {
    // Loop indefinitely, logging every minute
    loop {
        if let Err(e) = log_active_window().await {
            eprintln!("Error logging active window: {}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

async fn log_active_window() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let datetime_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let file_path = format!("./data/{}.csv", date_str);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)?;

    // Write CSV header if the file is new/empty
    if file.metadata()?.len() == 0 {
        writeln!(file, "DateTime,ProcessName,WindowTitle")?;
    }

    match get_active_window() {
        Ok(active_window) => {
            let mut system = System::new_all();
            system.refresh_all();

            let process_name = if let Some(process) = system.process(Pid::from_u32(active_window.process_id as u32)) {
                process.name().to_string()
            } else {
                format!("Unknown (PID: {})", active_window.process_id)
            };

            writeln!(
                file,
                "{},{},{}",
                datetime_str, process_name, active_window.title
            )?;
            println!("Logged: {} - {} - {}", datetime_str, process_name, active_window.title);
        }
        Err(e) => {
            eprintln!("Could not get active window: {:?}", e);
            writeln!(file, "{},{},{}", datetime_str, "Error", format!("{:?}", e))?;
        }
    }

    Ok(())
}
