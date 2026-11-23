# btrfs

一种基于写时复制（COW）原理的 Linux 文件系统。

## Usage
- `sudo btrfs su c 指向子卷的路径` : 创建子卷
- `sudo btrfs su l 指向挂载点的路径` : 列出子卷
- `sudo btrfs f df 指向挂载点的路径` : 显示空间使用情况信息
- `sudo btrfs qu e 指向子卷的路径` : 启用配额（quota）
- `sudo btrfs qg s 指向子卷的路径` : 显示配额（quota）

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs.html
