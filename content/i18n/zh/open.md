# open

打开文件、目录和应用程序。

## Usage
- `open 文件名.扩展名` : 使用系统关联的应用程序打开文件
- `open -a "应用程序名"` : 运行图形化的 macOS 应用程序
- `open -b com.domain.application` : 运行指定包标识符的图形化 macOS 应用程序（请参阅`OSascript`命令，查询如何获取应用程序的包标识符）
- `open .` : 在"访达（finder）"中打开当前文件夹
- `open -R 路径/到/文件` : 打开"访达（finder）", 并且给出指定文件
- `open *.扩展名` : 使用系统默认应用程序，打开当前目录中所有给定扩展名的文件
- `open -n -b com.domain.application` : 通过包标识符打开应用程序的新实例

## Links
- More information: https://keith.github.io/xcode-man-pages/open.1.html
