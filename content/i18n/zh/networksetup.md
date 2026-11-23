# networksetup

网络系统首选项配置工具。

## Usage
- `networksetup -listallnetworkservices` : 列出可用的网络服务源（以太网、Wi-Fi、蓝牙等）
- `networksetup -getinfo "Wi-Fi"` : 显示特定网络设备的配置信息
- `networksetup -getairportnetwork en0` : 获取当前连接的 Wi-Fi 网络名称（Wi-Fi 设备通常为 en0 或 en1）
- `networksetup -setairportnetwork en0 ssid 密码` : 连接到给定的 Wi-Fi 网络
