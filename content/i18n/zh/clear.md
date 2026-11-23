# clear

清空终端的屏幕。

## Usage
- `clear` : 清空屏幕（相当于在 Bash shell 中按 `<Ctrl l>` 键）
- `clear -x` : 清空屏幕但保留终端的回滚缓冲区
- `clear -T type_of_terminal` : 指明要清空的终端类型（默认为环境变量 `$TERM` 的值）
- `clear -V` : 显示 `clear` 使用的 `ncurses` 版本

## Links
- More information: https://manned.org/clear
