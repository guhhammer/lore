# wsl

从命令行管理适用于 Linux 的 Windows 子系统。

## Usage
- `wsl shell_命令` : 启动 Linux Shell（在默认发行版中）
- `wsl -e 命令 命令参数` : 在不使用 Shell 的情况下运行 Linux 命令
- `wsl -d 发行版 shell_命令` : 指定特定的发行版
- `wsl -l` : 列出所有可用发行版
- `wsl --export 发行版 路径\distro_fs.tar` : 将发行版导出到 `.tar` 文件
- `wsl --import 发行版 路径\安装位置 路径\distro_fs.tar` : 从 `.tar` 文件导入发行版
- `wsl --set-version 发行版 版本` : 更改指定发行版的版本
- `wsl --shutdown` : 关闭适用于 Linux 的 Windows 子系统

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
