# clear

清除終端機畫面。

## Usage
- `clear` : 清除畫面
- `clear -x` : 清除畫面，但保留終端機的捲動緩衝區（相當於在 Bash 中按下 `<Ctrl l>`）
- `clear -T 終端機類型` : 指定要清除的終端機類型（預設為環境變數 `$TERM` 的值）
- `clear -V` : 顯示 `clear` 所使用的 `ncurses` 版本

## Links
- More information: https://manned.org/clear
