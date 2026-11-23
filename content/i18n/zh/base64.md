# base64

将文件或标准输入编码到 Base64 或从 Base64 解码为标准输出。

## Usage
- `base64 路径/到/文件` : 编码一个文件
- `base64 -w 0|76|... 路径/到/文件` : 按特定宽度包装编码输出（“0”表示禁用包装）
- `base64 -d 路径/到/文件` : 解码一个文件
- `某指令 | base64` : 从标准输入编码
- `某指令 | base64 -d` : 将标准输入解码

## Links
- More information: https://manned.org/base64
