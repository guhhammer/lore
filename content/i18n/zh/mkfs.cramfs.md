# mkfs.cramfs

创建一个 ROM 文件系统，放置在分区内。

## Usage
- `mkfs.cramfs /dev/sdb1` : 在设备 b 的第 1 个分区内创建一个 ROM 文件系统（`sdb1`）
- `mkfs.cramfs -n volume_name /dev/sdb1` : 创建一个带有卷名的 ROM 文件系统

## Links
- More information: https://manned.org/mkfs.cramfs
