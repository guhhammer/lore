# qemu-img

创建和操作 Quick Emulator 虚拟硬盘镜像。

## Usage
- `qemu-img create 镜像名称.img gigabytesG` : 创建一个指定大小（以 GB 为单位）的磁盘镜像
- `qemu-img info 镜像名称.img` : 显示有关磁盘镜像的信息
- `qemu-img resize 镜像名称.img gigabytesG` : 增加或减少镜像大小
- `qemu-img map 镜像名称.img` : 导出指定磁盘镜像每个扇区的分配状态
- `qemu-img convert -f vmdk -O qcow2 路径/到/文件.vmdk 路径/到/文件.qcow2` : 将 VMware 的 `.vmdk` 磁盘镜像转换为 KVM 的 `.qcow2` 磁盘镜像

## Links
- More information: https://qemu.readthedocs.io/en/master/tools/qemu-img.html
