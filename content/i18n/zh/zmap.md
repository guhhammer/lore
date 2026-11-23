# zmap

用于全网范围调查的快速、开源网络扫描器。

## Usage
- `zmap SUBNETS -p 端口` : 扫描一个子网或整个 IPv4 地址空间上的指定 TCP 端口（默认：80）
- `zmap -p 端口1, 端口2-端口3,... SUBNETS` : 在一个子网中扫描指定的端口或端口范围
- `zmap -o 路径/到/输出文件.csv -f "saddr, daddr, sport, dport" SUBNETS` : 将结果输出到包含自定义字段的 CSV 文件中
- `zmap -r 每秒数据包数 SUBNETS` : 将扫描速率限制为每秒指定数量的数据包
- `zmap -d SUBNETS` : 执行一次不发送任何数据包的空跑
- `zmap -b 路径/到/阻止列表.txt SUBNETS` : 使用 CIDR 表示法的阻止列表文件来排除子网
- `zmap -S 源_ip SUBNETS` : 为扫描数据包设置指定的源 IP 地址
- `zmap -n 1000 SUBNETS -p 端口1, 端口2-端口3` : 限制要探测的目标数量或百分比（例如 1000 个 IP/端口组合）

## Links
- More information: https://manned.org/zmap
