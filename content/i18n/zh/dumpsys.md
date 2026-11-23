# dumpsys

提供关于 Android 系统服务的信息。 此命令只能通过 `adb shell` 使用。

## Usage
- `dumpsys` : 获取所有系统服务的诊断输出
- `dumpsys 服务` : 获取指定系统服务的诊断输出
- `dumpsys -l` : 列出 `dumpsys` 可以提供的所有服务信息
- `dumpsys 服务 -h` : 列出服务的指定服务参数
- `dumpsys --skip 服务` : 从诊断输出中排除指定服务
- `dumpsys -t 秒数` : 指定超时时间，以秒为单位（默认为 10s）

## Links
- More information: https://developer.android.com/tools/dumpsys
