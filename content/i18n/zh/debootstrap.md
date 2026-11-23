# debootstrap

创建一个基本的 `Debian` 系统。

## Usage
- `sudo debootstrap stable path/to/debian-root/ http://deb.debian.org/debian` : 在 `debian-root` 目录中创建一个 `Debian` 稳定分支系统
- `sudo debootstrap focal path/to/focal-root/ file:///path/to/mirror/` : 使用本地镜像在 `focal-root` 目录中创建一个 `Ubuntu 20.04` 系统
- `sudo chroot path/to/root` : 切换到可引导系统
- `ls /usr/share/debootstrap/scripts/` : 列出可用的版本

## Links
- More information: https://wiki.debian.org/Debootstrap
