# chezmoi

一個用 Go 語言寫的 dotfile 管理工具。

## Usage
- `chezmoi init` : 初始化 chezmoi
- `chezmoi add 檔案/完整/路徑` : 叫 chezmoi 管理一個 dotfile
- `chezmoi edit 檔案/完整/路徑` : 編輯一個已管理的 dotfile
- `chezmoi diff` : 檢視 chezmoi 所做的更動
- `chezmoi -v apply` : 套用所做的更動
- `chezmoi init https://example.com/path/to/repository.git` : 用一個已存在的 git repository 來初始化 chezmoi
- `chezmoi update` : 從遠端的 repository 獲取所做的更動

## Links
- More information: https://www.chezmoi.io/reference/
