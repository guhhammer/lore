# timedatectl

控制系统时间和日期。

## Usage
- `timedatectl` : 检查当前系统时钟时间
- `timedatectl set-time "yyyy-MM-dd hh:mm:ss"` : 直接设置系统时钟的本地时间
- `timedatectl list-timezones` : 列出可用时区
- `timedatectl set-timezone 时区` : 设置系统时区
- `timedatectl set-ntp on` : 启用网络时间协议（NTP）同步
- `timedatectl set-local-rtc 1` : 将硬件时钟时间标准更改为本地时间

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/timedatectl.html
