# zfs

管理 ZFS 文件系统。

## Usage
- `zfs list` : 列出所有可用的 ZFS 文件系统
- `zfs create 存储池名称/文件系统名称` : 创建一个新的 ZFS 文件系统
- `zfs destroy 存储池名称/文件系统名称` : 删除一个 ZFS 文件系统
- `zfs snapshot 存储池名称/文件系统名称@快照名称` : 创建一个 ZFS 文件系统的快照
- `zfs set compression=on 存储池名称/文件系统名称` : 在文件系统上启用压缩
- `zfs set mountpoint=/我的/挂载/路径 存储池名称/文件系统名称` : 更改文件系统的挂载点

## Links
- More information: https://manned.org/zfs
