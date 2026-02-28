# vigil

## vigil Installer

This installer automates the setup and configuration of the `vigil` application, which provides integrated management for servers and loggers.

### Features

*   Automatically downloads the latest `vigil` and `vigil-logger` binaries from GitHub Releases.
*   Sets up `vigil` to start automatically on system boot (via systemd on Linux, launchd on macOS, or Startup folder on Windows).
*   Places binaries and configuration files in user-specific directories, **requiring no `sudo` or administrator privileges**.

### Installation

To install `vigil`, simply run the installer binary for your operating system.

**1. Download the Installer**

[Link to GitHub Releases where pre-built installers would be. (Placeholder for now)]

**2. Run the Installer**

Open your terminal or command prompt, navigate to the directory where you downloaded the installer, and execute it:

**On Linux/macOS:**
```bash
./vigil-installer
```

**On Windows:**
```bash
.\vigil-installer.exe
```

The installer will automatically detect your operating system and architecture, download the appropriate binaries, and configure `vigil` for auto-start.

### Configuration

`vigil` uses a `config.toml` file for its settings.
After installation, this file is typically located in your user's configuration directory:

*   **Linux:** `~/.config/vigil/config.toml`
*   **macOS:** `~/Library/Application Support/com.44103.vigil/config.toml`
*   **Windows:** `%APPDATA%\44103\vigil\config.toml` (e.g., `C:\Users\<username>\AppData\Roaming\44103\vigil\config.toml`)

You can edit this file to customize settings such as `log_output_path` and `monitor_interval_secs`.

#### Applying Changes

After modifying `config.toml`, you must **restart the Vigil service** for the changes to take effect. Choose one of the following methods for your operating system:

*   **Linux:**
    *   **Command:** `systemctl --user restart vigil.service`
    *   **Manual:** Restart your computer.
*   **macOS:**
    *   **Command:** 
        ```bash
        launchctl unload ~/Library/LaunchAgents/com.44103.vigil.plist
        launchctl load ~/Library/LaunchAgents/com.44103.vigil.plist
        ```
    *   **Manual:** Restart your computer.
*   **Windows:**
    *   **Command (PowerShell):** `Stop-Process -Name "vigil" -Force; start-process "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\vigil.exe"`
    *   **Manual:** Restart your computer, or find `vigil.exe` in the Task Manager, end the process, and run it again from the Startup folder.

### Uninstallation

To uninstall `vigil`, you will need to manually remove the installed files and disable the auto-start services:

*   **Remove binaries and data:** Delete the `vigil` directory from your user's application data directory (e.g., `~/.local/share/vigil` on Linux).
*   **Disable auto-start service:**
    *   **Linux:** `systemctl --user disable --now vigil.service` and remove `~/.config/systemd/user/vigil.service`.
    *   **macOS:** `launchctl unload ~/Library/LaunchAgents/com.44103.vigil.plist` and remove `~/Library/LaunchAgents/com.44103.vigil.plist`.
    *   **Windows:** Remove `vigil.exe` from your Startup folder.