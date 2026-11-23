# xz

解压缩 XZ 和 LZMA 文件。

## Usage
- `xz 路径/到/文件` : 使用 xz 压缩文件
- `xz -d 路径/到/文件.xz` : 解压 XZ 文件
- `xz -F lzma 路径/到/文件` : 使用 lzma 压缩文件
- `xz -d -F lzma 路径/到/文件.lzma` : 解压 LZMA 文件
- `xz -d -c 路径/到/文件.xz` : 解压文件并输出到 `stdout`（暗示 `--keep`）
- `xz -k 路径/到/文件` : 压缩文件但不删除原文件
- `xz -0 路径/到/文件` : 使用最快方式压缩文件
- `xz -9 路径/到/文件` : 使用最好方式压缩文件

## Links
- More information: https://manned.org/xz
