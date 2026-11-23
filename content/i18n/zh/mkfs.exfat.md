# mkfs.exfat

在分区内创建一个 exFAT 文件系统。

## Usage
- `sudo mkfs.exfat /dev/sdb1` : 在设备 b 的分区 1 内创建一个 exFAT 文件系统（`sdb1`）
- `sudo mkfs.exfat -L volume_name /dev/sdXY` : 创建一个带有卷名的文件系统
- `sudo mkfs.exfat -U volume_id /dev/sdXY` : 创建一个带有卷 ID 的文件系统

## Links
- More information: https://manned.org/mkfs.exfat
