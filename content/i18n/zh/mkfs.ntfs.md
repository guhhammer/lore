# mkfs.ntfs

在分区内创建一个 NTFS 文件系统。

## Usage
- `sudo mkfs.ntfs /dev/sdb1` : 在设备 b 的分区 1 内创建一个 NTFS 文件系统（`sdb1`）
- `sudo mkfs.ntfs -L volume_label /dev/sdb1` : 创建一个带有卷标签的文件系统
- `sudo mkfs.ntfs -U uuid /dev/sdb1` : 创建一个带有特定 UUID 的文件系统

## Links
- More information: https://manned.org/mkfs.ntfs
