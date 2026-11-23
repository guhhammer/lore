# wsl

透過命令行管理 Windows 子系統 Linux 版 (WSL)。

## Usage
- `wsl shell 命令` : 在預設發行版下，開啟 Linux shell
- `wsl -e 命令 命令參數` : 不使用 shell 執行 Linux 命令
- `wsl -d 發行版 shell 命令` : 指定發行版
- `wsl -l` : 列出可用的發行版
- `wsl --export 發行版 檔案\完整\distro_fs.tar` : 將發行版匯出至 `.tar` 檔
- `wsl --import 發行版 檔案\完整\安裝位置 檔案\完整\distro_fs.tar` : 從 `.tar` 檔匯入發行版
- `wsl --set-version 發行版 WSL 版本` : 變更執行特定發行版的 Windows 子系統 Linux 版本
- `wsl --shutdown` : 終止執行 Windows 子系統 Linux 版

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
