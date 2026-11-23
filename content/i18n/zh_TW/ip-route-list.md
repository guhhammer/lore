# ip route list

用於顯示 IP 路由表管理的子指令。

## Usage
- `ip r l` : 顯示 `main` 路由表
- `ip r l t main|254` : 顯示主要路由表（與第一個範例相同）
- `ip r l t local|255` : 顯示本機路由表
- `ip r l t all|unspec|0` : 顯示所有路由表
- `ip r l dev ethX` : 僅列出指定裝置的路由
- `ip r l s link` : 列出指定範圍內的路由
- `ip r l c` : 顯示路由快取
- `ip -6|-4 r` : 僅顯示 IPv6 或 IPv4 路由

## Links
- More information: https://manned.org/ip-route
