# diskutil

用于管理本地磁盘和卷的实用程序。

## Usage
- `diskutil list` : 列出所有当前可用的磁盘、分区和已装入的卷
- `diskutil repairVolume 目标卷文件` : 修复卷的文件系统数据结构
- `diskutil unmountDisk 目标卷文件` : 卸载卷
- `diskutil eject /dev/ 光驱文件名` : 弹出 CD/DVD（先卸载）

## Links
- More information: https://keith.github.io/xcode-man-pages/diskutil.8.html
