# wg

管理 WireGuard 接口配置。

## Usage
- `sudo wg` : 检查当前激活接口的状态
- `wg genkey` : 生成新的私钥
- `wg <路径/到/私钥 pubkey >路径/到/公钥` : 从私钥生成公钥
- `wg genkey | tee 路径/到/私钥 | wg pubkey >路径/到/公钥` : 同时生成公钥和私钥
- `sudo wg showconf wg0` : 展示 WireGuard 接口的当前配置

## Links
- More information: https://www.wireguard.com/quickstart/
