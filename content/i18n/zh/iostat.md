# iostat

报告设备和分区的统计信息。

## Usage
- `iostat` : 显示系统启动以来的 CPU 和磁盘统计报告
- `iostat -m` : 以 MB 为单位显示 CPU 和磁盘统计报告
- `iostat -c` : 显示 CPU 统计信息
- `iostat -N` : 显示包含磁盘名称（含 LVM）的磁盘统计信息
- `iostat -xN sda` : 显示设备"sda"的扩展磁盘统计信息（包含磁盘名称）
- `iostat 2` : 每 2 秒显示一次 CPU 和磁盘的增量统计报告

## Links
- More information: https://manned.org/iostat
