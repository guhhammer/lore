# qemu-system-i386

模拟 `i386` 架构。

## Usage
- `qemu-system-i386 -hda image_name.img -m 4096` : 从一个模拟 `i386` 架构的镜像启动
- `qemu-system-i386 -hda image_name.img -m 4096 -cdrom os_image.iso -boot d` : 从一个实时 ISO 镜像启动 QEMU 实例
- `qemu-system-i386 -hda /dev/storage_device -m 4096` : 从物理设备启动（例如从 USB 启动以测试可引导介质）
- `qemu-system-i386 -hda image_name.img -m 4096 -nographic` : 不启动 VNC 服务器
- `<Ctrl a><x>` : 退出无图形界面的 QEMU
- `qemu-system-i386 -M help` : 列出支持的机器类型

## Links
- More information: https://www.qemu.org/docs/master/system/target-i386.html
