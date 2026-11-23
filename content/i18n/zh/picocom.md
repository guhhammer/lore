# picocom

模拟串行端口的极简程序。

## Usage
- `sudo picocom /dev/ttyXYZ` : 使用默认波特率 9600 连接到串行控制台
- `sudo picocom /dev/ttyXYZ -b 波特率` : 以指定波特率连接到串行控制台
- `sudo picocom /dev/ttyXYZ --imap lfcrlf` : 映射特殊字符（例如：将 `LF` 映射为 `CRLF`）
- `<Ctrl a><Ctrl x>` : 退出 picocom
- `picocom -h` : 显示帮助

## Links
- More information: https://manned.org/picocom
