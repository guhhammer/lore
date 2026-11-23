# openvpn3

OpenVPN 3 Linux 客户端。

## Usage
- `openvpn3 session-start -c 路径/到/config.conf` : 打开一个新的 VPN 会话
- `openvpn3 sessions-list` : 列出已建立的会话
- `openvpn3 session-manage -c 路径/到/config.conf -D` : 断开当前建立的以给定配置开始的会话
- `openvpn3 config-import -c 路径/到/config.conf` : 导入 VPN 配置
- `openvpn3 configs-list` : 列出导入的配置

## Links
- More information: https://community.openvpn.net/openvpn/wiki/OpenVPN3Linux
