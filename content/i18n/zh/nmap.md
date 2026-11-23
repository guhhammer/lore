# nmap

网络探索工具和安全/端口扫描程序。 某些功能（如 SYN 扫描）仅当以 root 权限运行 `nmap` 时才能激活。

## Usage
- `nmap -v1|2|3 ip或主机名` : 使用不同的详细（verbosity）级别扫描远程主机的前 1000 个端口
- `nmap -T5 -sn 192.168.0.0/24|ip或主机名1,ip或主机名2,...` : 对整个子网或单个主机进行非常激进的 ping 扫描
- `sudo nmap -A -iL 路径/到/文件.txt` : 对来自文件的主机启用操作系统检测、版本检测、脚本扫描和路由跟踪
- `nmap -p 端口1,端口2,... ip或主机名1,ip或主机名2,...` : 扫描特定的端口列表（使用 `-p-` 扫描从 1 到 65535 的所有端口）
- `nmap -sC -sV -oA top-1000-ports ip或主机名1,ip或主机名2,...` : 使用默认 NSE 脚本执行前 1000 个端口的服务和版本检测，将结果（`-oA`）写入输出文件
- `nmap --script "default and safe" ip或主机名1,ip或主机名2,...` : 使用 `default and safe` NSE 脚本谨慎地扫描目标
- `nmap --script "http-*" ip或主机名1,ip或主机名2,... -p 80,443` : 使用所有可用的 `http-*` NSE 脚本扫描在标准端口 80 和 443 上运行的 Web 服务器
- `sudo nmap -T0 -D 诱饵ip1,诱饵ip2,... --source-port 53 -f --data-length 16 -Pn ip或主机名` : 尝试通过使用极慢扫描（`-T0`）、诱饵源地址（`-D`）、分段（fragmented）数据包、随机数据和其他方法来规避 IDS/IPS 检测

## Links
- More information: https://nmap.org/book/man.html
