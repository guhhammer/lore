# apt-file

在 APT 软件包中查找文件，其中也包括未安装的软件。

## Usage
- `sudo apt update` : 更新元数据的数据库
- `apt-file find 文件路径` : 查找包含指定文件或路径的软件包
- `apt-file list 软件包名` : 列出指定包的内容
- `apt-file find -x 正则表达式` : 查找符合给定 `pattern` 中`regex`式的软件包

## Links
- More information: https://manned.org/apt-file
