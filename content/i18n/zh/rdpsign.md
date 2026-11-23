# rdpsign

用于签名远程桌面协议（RDP）文件的工具。

## Usage
- `rdpsign 文件路径.rdp` : 为一个 RDP 文件签名
- `rdpsign 文件路径.rdp /sha265 哈希值` : 使用一个指定的 sha256 哈希值为 RDP 文件签名
- `rdpsign 文件路径.rdp /q` : 启用静默输出
- `rdpsign 文件路径.rdp /v` : 显示详细的信息、警告和状态
- `rdpsign 文件路径.rdp /l` : 在不更新文件的情况下将输出显示到标准输出来测试签名
