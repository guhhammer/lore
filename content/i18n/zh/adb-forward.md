# adb forward

通过无线连接到一个 Android 设备。

## Usage
- `adb forward tcp:本地_端口 tcp:远程_端口` : 转发一个 TCP 端口
- `adb forward --list` : 列出所有转发规则
- `adb forward --remove tcp:本地_端口` : 移除转发规则
- `adb forward --remove-all` : 移除所有转发规则

## Links
- More information: https://developer.android.com/tools/adb
