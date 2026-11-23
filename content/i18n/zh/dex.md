# dex

DesktopEntry Execution 是一个用于生成和执行 Application 类型的 DesktopEntry 文件的程序。

## Usage
- `dex -a` : 执行自动启动文件夹中的所有程序
- `dex -a -s 路径/到/目录1:路径/到/目录2:路径/到/目录3:` : 执行指定文件夹中的所有程序
- `dex -a -e GNOME` : 预览在 GNOME 特定自动启动中会执行的程序
- `dex -a -d` : 预览在常规自动启动中会执行的程序
- `dex -p Name 路径/到/文件.desktop` : 预览 DesktopEntry 属性 `Name` 的值
- `dex -c 路径/到/文件.desktop` : 为当前目录中的程序创建 DesktopEntry
- `dex --term 终端 路径/到/文件.desktop` : 在指定终端中执行单个程序（桌面文件中设置了 `Terminal=true`）

## Links
- More information: https://github.com/jceb/dex#dex
