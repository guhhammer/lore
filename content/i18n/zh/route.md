# route

手动操作路由表。

## Usage
- `sudo route add "路由 ip 地址" "网关地址"` : 通过网关向目标添加路由
- `sudo route add "子网 ip/24" "网关地址"` : 通过网关向 子网 / 24 添加路由
- `sudo route -t add "路由 ip 地址/24" "网关地址"` : 在测试模式下运行（不做任何操作，只打印）
- `sudo route flush` : 删除所有路由
- `sudo route delete "路由 ip 地址/24"` : 删除特定路由
- `sudo route get "目标"` : 查找并显示目标的路由（主机名或 IP 地址）

## Links
- More information: https://keith.github.io/xcode-man-pages/route.8.html
