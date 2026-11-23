# eza

基於 `exa` 組建的現代化、持續維護的 `ls` 替代品。

## Usage
- `eza -1` : 每行列出一個檔案
- `eza -a` : 列出所有檔案，包括隱藏檔案
- `eza -al` : 以長格式列出所有檔案（包含權限、所有者、大小和修改日期）
- `eza -r -s size` : 列出檔案，最大的排在最上面
- `eza -lT -L 3` : 顯示檔案樹，深度為三層
- `eza -l -s modified` : 按修改日期排序列出檔案（從最舊的開始）
- `eza -lh --icons --git` : 列出檔案及其表頭、圖示和 Git 狀態
- `eza --git-ignore` : 不列出 `.gitignore` 中提到的檔案

## Links
- More information: https://github.com/eza-community/eza
