# pmset

配置 macOS 电源管理设置，就像在系统首选项 > 节能程序中一样。 修改设置的命令必须以 `sudo` 开头。

## Usage
- `pmset -g` : 显示当前电源管理设置
- `pmset -g batt` : 显示当前电源和电池电量
- `pmset displaysleepnow` : 立即让显示器进入休眠状态
- `sudo pmset -c displaysleep 0` : 当充电器通电时，将显示器设置为从不休眠
- `sudo pmset -b displaysleep 15` : 使用电池电源 15 分钟后将显示器设置为休眠
- `sudo pmset repeat wake MTWRF 09:00:00` : 安排计算机在每个工作日上午 9 点自动唤醒
- `sudo pmset -a displaysleep 10 disksleep 10 sleep 30 womp 1` : 还原为系统默认值

## Links
- More information: https://keith.github.io/xcode-man-pages/pmset.1.html
