# mkfs.ext4

在分区内创建一个 ext4 文件系统。

## Usage
- `sudo mkfs.ext4 /dev/sdb1` : 在设备 b 的分区 1 内创建一个 ext4 文件系统（`sdb1`）
- `sudo mkfs.ext4 -L volume_label /dev/sdb1` : 创建一个带有卷标签的 ext4 文件系统

## Links
- More information: https://manned.org/mkfs.ext4
