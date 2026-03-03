# vigil

## vigil インストーラー

このインストーラーは、以下のコンポーネントで構成される `vigil` システムのセットアップと設定を自動化します。
*   **`vigild`**: アクティブなウィンドウを監視し、API を提供するバックグラウンドサービス（デーモン）。
*   **`vigil`**: システムと対話するためのコマンドラインインターフェース（CLI）。
*   **`vigil-logger`**: ロギングコンポーネント。

### 機能

*   GitHub Releases から最新の `vigild`、`vigil`、および `vigil-logger` バイナリを自動的にダウンロードします。
*   システム起動時に `vigild` サービスが自動的に開始されるように設定します。
*   バイナリと設定ファイルをユーザー固有のディレクトリに配置するため、**`sudo` や管理者権限は不要**です。

### インストール

`vigil` をインストールするには、お使いのオペレーティングシステムに対応するインストーラーバイナリを実行するだけです。

**1. インストーラーのダウンロード**

[事前ビルドされたインストーラーが置かれる GitHub Releases へのリンク (現在はプレースホルダー)]

**2. インストーラーの実行**

ターミナルまたはコマンドプロンプトを開き、インストーラーをダウンロードしたディレクトリに移動して実行します。

**Linux/macOS の場合:**
```bash
./vigil-installer
```

**Windows の場合:**
```bash
.\vigil-installer.exe
```

インストーラーは自動的にオペレーティングシステムとアーキテクチャを検出し、適切なバイナリをダウンロードし、`vigil` の自動起動を設定します。

### 設定

`vigil` は `config.toml` ファイルで設定を管理します。
インストール後、このファイルは通常、ユーザーの設定ディレクトリにあります。

*   **Linux:** `~/.config/vigil/config.toml`
*   **macOS:** `~/Library/Application Support/com.44103.vigil/config.toml`
*   **Windows:** `%APPDATA%\44103\vigil\config.toml` (例: `C:\Users\<username>\AppData\Roaming\44103\vigil\config.toml`)

このファイルを編集して、`log_output_path` や `monitor_interval_secs` などの設定をカスタマイズできます。

#### 設定の反映

`config.toml` を変更した後、設定を反映させるには **Vigil デーモン (`vigild`) を再起動**する必要があります。お使いのオペレーティングシステムに合わせて、以下のいずれかの方法を選択してください。

*   **Linux:**
    *   **コマンド:** `systemctl --user restart vigil.service`
    *   **手動:** コンピュータを再起動します。
*   **macOS:**
    *   **コマンド:**
        ```bash
        launchctl unload ~/Library/LaunchAgents/com.44103.vigil.plist
        launchctl load ~/Library/LaunchAgents/com.44103.vigil.plist
        ```
    *   **手動:** コンピュータを再起動します。
*   **Windows:**
    *   **コマンド (PowerShell):** `Stop-Process -Name "vigild" -Force; start-process "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\vigild.exe"`
    *   **手動:** コンピュータを再起動するか、タスクマネージャーで `vigild.exe` を見つけてプロセスを終了し、スタートアップフォルダから再度実行してください。

### アンインストール

`vigil` をアンインストールするには、インストールされたファイルを手動で削除し、自動起動サービスを無効にする必要があります。

*   **バイナリとデータの削除:** ユーザーのアプリケーションデータディレクトリから `vigil` ディレクトリを削除します（例: Linux では `~/.local/share/vigil`）。
*   **自動起動サービスの無効化:**
    *   **Linux:** `systemctl --user disable --now vigil.service` を実行し、`~/.config/systemd/user/vigil.service` を削除します。
    *   **macOS:** `launchctl unload ~/Library/LaunchAgents/com.44103.vigil.plist` を実行し、`~/Library/LaunchAgents/com.44103.vigil.plist` を削除します。
    *   **Windows:** スタートアップフォルダから `vigil.exe` を削除します。
