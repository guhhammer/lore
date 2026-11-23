# duf

磁盘占用/空闲实用工具。

## Usage
- `duf` : 列出可访问设备
- `duf --all` : 列出所有（如伪文件系统，重复文件系统或不可访问的文件系统）
- `duf 路径/到/文件夹1 路径/到/文件夹2 ...` : 只显示指定的设备或挂载点
- `duf --sort size|used|avail|usage` : 根据指定条件排序输出
- `duf --only-fs|hide-fs tmpfs|vfat|ext4|xfs` : 显示或隐藏指定文件系统
- `duf --sort mountpoint|size|used|avail|usage|inodes|inodes_used|inodes_avail|inodes_usage|type|filesystem` : 根据键排序输出
- `duf --theme dark|light` : 更改主题（如果 `duf` 未能使用正确的主题）

## Links
- More information: https://github.com/muesli/duf#usage
