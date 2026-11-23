# aura

Aura 包管理器：一个安全且支持多语言的 Arch Linux 和 AUR 的包管理器。

## Usage
- `aura --aursync --both --search 包名|正则` : 从官方仓库和 AUR 检索包
- `aura --aursync 包名` : 从 AUR 安装一个软件包
- `aura --aursync --diff --sysupgrade --delmakedeps --unsuppress` : 以详细模式升级所有的 AUR 包并移除所有的编译依赖
- `aura --sync 包名` : 从官方仓库安装一个软件包
- `aura --sync --refresh --sysupgrade` : 同步并更新官方仓库的所有软件包
- `aura --downgrade 包名` : 使用包缓存降级一个软件包
- `aura --remove --recursive --unneeded 包名` : 移除一个软件包及其依赖
- `aura --orphans --abandon` : 移除孤儿包（作为依赖安装但现在不被任何包依赖）

## Links
- More information: https://github.com/fosskers/aura
