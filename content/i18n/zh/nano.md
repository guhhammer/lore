# nano

命令行文本编辑器。一个功能更强的 `Pico` 克隆版。

## Usage
- `nano` : 启动编辑器
- `nano -I` : 启动编辑器且不使用配置文件
- `nano 路径/到/文件1 路径/到/文件2 ...` : 打开指定文件，在关闭前一个文件后移动到下一个文件
- `nano +行号,列号 路径/到/文件` : 打开文件并将光标定位到指定行和列
- `nano -S 路径/到/文件` : 打开文件并启用软换行
- `nano -i 路径/到/文件` : 打开文件并使新行缩进对齐到上一行
- `nano -B 路径/到/文件` : 打开文件并在保存时创建备份文件（如 `路径/到/文件~`）
- `<Ctrl x>` : 退出 nano

## Links
- More information: https://nano-editor.org/dist/latest/nano.html
