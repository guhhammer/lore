# ip

显示/操作路由、设备、策略路由和隧道。 一些子命令（例如 `address`）有自己的使用文档。

## Usage
- `ip a` : 列出带有详细信息的接口
- `ip -br a` : 列出带有简要网络层信息的接口
- `ip -br l` : 列出带有简要链路层信息的接口
- `ip r` : 显示路由表
- `ip n` : 显示邻居（ARP 表）
- `sudo ip l s interface up|down` : 使接口启动/关闭
- `sudo ip a add|delete ip_address/mask dev interface` : 向接口添加/删除 IP 地址
- `sudo ip r a default via ip_address dev interface` : 添加默认路由

## Links
- More information: https://manned.org/ip.8
