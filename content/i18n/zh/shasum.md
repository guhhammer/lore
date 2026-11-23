# shasum

计算或检查加密 SHA 校验值。

## Usage
- `shasum 路径/到/文件1 路径/到/文件2 ...` : 计算一个或多个文件的 SHA1 校验值
- `shasum --algorithm 1|224|256|384|512|512224|512256 路径/到/文件1 路径/到/文件2 ...` : 使用指定算法计算一个或多个文件的 SHA 校验值
- `命令 | shasum` : 从标准输入中获取并计算 SHA1 校验值
- `shasum --algorithm 256 路径/到/文件1 路径/到/文件2 ... >路径/到/文件.sha256` : 计算并保存 SHA256 校验值列表到文件
- `shasum -c 路径/到/文件` : 读取包含 SHA 校验值和文件名的文件，并验证所有文件是否具有匹配的校验值（算法将被自动检测）
- `shasum -c --quiet 路径/到/文件` : 仅显示缺失文件或验证失败的消息
- `shasum --ignore-missing -c --quiet 路径/到/文件` : 仅显示验证失败的消息，忽略缺失的文件
- `echo '文件的已知 sha 校验值  路径/到/文件' | shasum -c` : 检查文件的已知 SHA 校验值

## Links
- More information: https://manned.org/shasum
