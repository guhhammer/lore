# dnf

RHEL, Fedora 和 CentOS 的软件包管理工具（yum 的替代品）。

## Usage
- `sudo dnf upgrade` : 更新已安装的包到最新可用版本
- `dnf search 关键词1 关键词2 ...` : 通过关键词搜索包
- `dnf info 包` : 显示软件包的描述
- `sudo dnf install 包1 包2 ...` : 安装软件包（使用 `-y` 自动确认所有提示）
- `sudo dnf remove 包1 包2 ...` : 删除软件包
- `dnf list --installed` : 列出已安装的包
- `dnf provides 命令` : 查找哪些包提供给定命令
- `dnf history` : 查看所有过去的操作

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
