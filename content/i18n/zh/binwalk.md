# binwalk

固件分析工具。

## Usage
- `binwalk 二进制文件` : 扫描一个二进制文件
- `binwalk -e -C 输出目录 二进制文件` : 解压一个二进制文件并指定输出目录
- `binwalk -e -M -d 2 二进制文件` : 递归解压一个二进制文件并限制递归深度为 2
- `binwalk -D 'png image:png' 二进制文件` : 解压一个二进制文件并指定文件签名
- `binwalk -E -J 二进制文件` : 分析一个二进制文件的熵，用与文件相同的名字和 `.png` 后缀保存绘图
- `binwalk -E -B -A 二进制文件` : 在单条命令中组合熵、签名和操作码分析

## Links
- More information: https://manned.org/binwalk
