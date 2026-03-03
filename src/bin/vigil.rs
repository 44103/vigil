use vigil::config::APP_VERSION;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-v" => {
                println!("vigil version {}", APP_VERSION);
                return;
            }
            "status" => {
                println!(
                    "Vigil Status: Maybe it's watching you... or maybe it's just out for a quick cup of tea ☕."
                );
                println!("(Actual status check via API is coming in the next update!)");
                return;
            }
            _ => {
                println!("Unknown command. Available commands: status, --version");
                return;
            }
        }
    }

    println!("Vigil CLI Tool");
    println!("Usage: vigil <COMMAND>");
    println!(
        "
Commands:"
    );
    println!("  status    Check the status of the vigil daemon");
    println!("  --version Display version information");
}
