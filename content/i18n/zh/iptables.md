# iptables

可用于配置 Linux 内核防火墙提供的过滤表、规则链和规则的程序。 使用 `ip6tables` 来设置 IPv6 流量规则。

## Usage
- `sudo iptables -vnL --line-numbers` : 查看过滤表的规则链、规则以及数据包/字节计数器
- `sudo iptables -P 规则链 规则` : 设定规则链策略规则
- `sudo iptables -A 规则链 -s ip_address -j 规则` : 追加规则到 IP 的规则链策略
- `sudo iptables -A 规则链 -s ip_address -p 协议 --dport 端口 -j 规则` : 追加规则到 IP 的规则链策略（考虑协议与端口）
- `sudo iptables -t nat -A POSTROUTING -s 192.168.0.0/24 -j MASQUERADE` : 添加 NAT 规则，将来自 `192.168.0.0/24` 子网的所有流量转换为主机的公共 IP
- `sudo iptables -D 规则链 规则所在行号` : 删除规则链中的规则

## Links
- More information: https://manned.org/iptables
