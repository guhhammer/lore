# btrfs device

管理 btrfs 文件系统中的设备。

## Usage
- `sudo btrfs d a 指向设备1的路径 指向设备2的路径 ... 指向 btrfs 文件系统的路径` : 将一个或多个设备添加到 btrfs 文件系统中
- `sudo btrfs d rem 指向设备1的路径|设备 ID1 指向设备2的路径|设备 ID2 ...` : 从 btrfs 文件系统中删除设备
- `sudo btrfs d st 指向 btrfs 文件系统的路径` : 显示错误统计
- `sudo btrfs d sc -d` : 扫描所有磁盘并将所有检测到的 btrfs 文件系统通知内核
- `sudo btrfs d u 指向 btrfs 文件系统的路径` : 显示详细的每个磁盘的空间分配统计信息

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-device.html
