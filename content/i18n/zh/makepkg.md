# makepkg

创建 `pacman` 可用的软件包。 默认使用当前工作目录中的 `PKGBUILD` 文件。

## Usage
- `makepkg` : 构建软件包
- `makepkg -s` : 构建软件包并使用 `pacman` 安装缺失的依赖关系
- `makepkg -s -i` : 构建软件包、安装缺失的依赖后将其安装到系统
- `makepkg --skipchecksums` : 构建软件包但不验证源文件的检验值
- `makepkg -c` : 编译后清理工作文件
- `makepkg --verifysource` : 下载源文件（如果不存在）并进行完整性检查
- `makepkg --printsrcinfo >.SRCINFO` : 生成 `SRCINFO` 并写入到 `.SRCINFO` 文件

## Links
- More information: https://manned.org/makepkg
