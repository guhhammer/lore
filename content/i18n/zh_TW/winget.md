# winget

Windows 套件管理員的命令列工具。

## Usage
- `winget add 套件` : 安裝一個套件
- `winget rm package` : 刪除一個套件（註：可以用 `remove` 代替 `uninstall`）
- `winget show 套件` : 顯示指定套件的相關資訊
- `winget search 套件` : 搜尋指定套件
- `winget upgrade -r` : 升級所有套件至最新版本
- `winget ls -s winget` : 列出所有可由 `winget` 管理的已安裝套件
- `winget import|export --import-file|--output 檔案\完整\路徑` : 從檔案匯入套件，或將已安裝的套件匯出至檔案
- `winget validate manifest檔案\完整\路徑` : 在提交 PR 到 winget-pkgs 存儲庫之前，請驗證 manifest 檔

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
