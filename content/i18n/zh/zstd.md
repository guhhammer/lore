# zstd

使用 Zstandard 压缩来压缩 / 解压文件。

## Usage
- `zstd file` : 将一个文件压缩到一个 `.zst` 后缀的压缩文件中
- `zstd --decompress file.zst` : 解压缩一个文件
- `zstd --decompress --stdout file.zst` : 将文件解压缩到标准输出（`stdout`）
- `zstd -level file` : 使用指定的压缩等级来压缩一个文件.0 = 最差，19 = 最好（默认等级是 3）
- `zstd --ultra -level file` : 使用更多内存（解压或压缩时）来得到更高的压缩比

## Links
- More information: https://manned.org/zstd
