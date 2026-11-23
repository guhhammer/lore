# rg

Ripgrep，一款遞迴的行導向搜尋工具。 旨在成為 `grep` 的更快替代品。

## Usage
- `rg 模式` : 在目前目錄中遞迴搜尋匹配模式的內容（`regex`）
- `rg 模式 路徑/至/檔案或目錄` : 在指定檔案或目錄中遞迴搜尋匹配模式的內容
- `rg -F -- 字串` : 搜尋字面字串模式
- `rg -. --no-ignore 模式` : 包含隱藏檔案以及 `.gitignore` 中列出的條目
- `rg 模式 -g 檔名萬用字元模式` : 僅搜尋檔名匹配萬用字元（`glob`）模式的檔案（例如 `README.*`）
- `rg --files | rg 模式` : 遞迴列出目前目錄中匹配模式的檔名
- `rg -l 模式` : 僅列出匹配的檔案（適用於管線傳遞給其他命令）
- `rg -v 模式` : 顯示不匹配該模式的行

## Links
- More information: https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md
