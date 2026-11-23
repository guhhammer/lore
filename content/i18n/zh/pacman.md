# pacman

Arch Linux 的软件包管理器工具。

## Usage
- `sudo pacman -Syu` : 同步并更新所有软件包
- `sudo pacman -S 软件包` : 安装一个新的软件包
- `sudo pacman -Rs 软件包` : 删除一个软件包及其依赖
- `pacman -Q` : 列出已安装的软件包和版本
- `pacman -Qe` : 仅列出明确安装的软件包和版本
- `pacman -Qo 文件名` : 查找哪个包拥有某个文件
- `sudo pacman -Scc` : 清空软件包缓存以释放空间

## Links
- More information: https://manned.org/pacman.8
