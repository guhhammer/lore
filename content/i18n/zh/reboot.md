# reboot

重新启动系统。

## Usage
- `reboot` : 重启系统
- `reboot -p` : 关闭系统（等同于 `poweroff`）
- `reboot --halt` : 停止系统（终止所有进程并关闭 CPU，等同于 `halt`）
- `reboot -f` : 立即重启，而无需正常关闭
- `reboot -w` : 仅写入 wtmp 关机日志条目而不重启系统

## Links
- More information: https://manned.org/reboot.8
