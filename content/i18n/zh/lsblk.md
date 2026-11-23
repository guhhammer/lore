# lsblk

列出设备信息。

## Usage
- `lsblk` : 以树状格式列出所有存储设备
- `lsblk -a` : 同时列出空设备
- `lsblk -b` : 以字节为单位而不是以人类可读的格式打印 SIZE 列
- `lsblk -f` : 输出文件系统信息
- `lsblk -i` : 使用 ASCII 字符进行树形格式化
- `lsblk -t` : 输出块设备的拓扑结构
- `lsblk -e 1,7,...` : 排除由逗号分隔的主要设备编号列表指定的设备
- `lsblk -o 名称,序列号,型号,传输方式,类型,大小,文件系统类型,挂载点,...` : 使用逗号分隔的列列表显示自定义摘要

## Links
- More information: https://manned.org/lsblk
