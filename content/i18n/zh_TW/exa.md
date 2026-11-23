# exa

`ls` 的現代替代品（列出目錄內容）。

## Usage
- `exa -1` : 每行列出一個檔案
- `exa -a` : 列出所有檔案，包括隱藏檔案
- `exa -l -a` : 以長格式列出所有檔案（包含權限、所有者、大小和修改日期）
- `exa -r -s size` : 列出檔案，最大的排在最上面
- `exa -l -T -L 3` : 顯示檔案樹，深度為三層
- `exa -l -s modified` : 按修改日期排序列出檔案（從最舊的開始）
- `exa -l -h --icons --git` : 列出檔案及其表頭、圖示和 Git 狀態
- `exa --git-ignore` : 不列出 `.gitignore` 中提到的檔案

## Links
- More information: https://github.com/ogham/exa#command-line-options
