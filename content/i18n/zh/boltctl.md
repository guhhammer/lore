# boltctl

控制雷电（thunderbolt）设备。

## Usage
- `boltctl` : 列出已连接并授权的设备
- `boltctl list` : 列出已连接的设备，且包含未授权的设备
- `boltctl authorize 设备uuid` : 临时授权一个设备
- `boltctl enroll 设备uuid` : 授权并记住一个设备
- `boltctl forget 设备uuid` : 取消一个设备的授权
- `boltctl info 设备uuid` : 显示一个设备的详细信息

## Links
- More information: https://manned.org/boltctl
