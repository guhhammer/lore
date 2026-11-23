# aurman

用来构建和安装 AUR 包的 Arch Linux 实用工具。

## Usage
- `aurman -S -y -u` : 同步并更新所有包
- `aurman -S -y -u --noedit` : 同步并更新所有包但不显示 `PKGBUILD` 文件的变动
- `aurman -S 包名` : 安装一个新包
- `aurman -S --noedit 包名` : 安装一个新包但不显示 `PKGBUILD` 文件的变动
- `aurman -S --noedit --noconfirm 包名` : 无确认提示安装一个新包
- `aurman -S -s 关键字` : 在官方仓库和 AUR 的包数据库中查找关键字
- `aurman --remove --recursive --nosave 包名` : 移除一个包及其依赖
- `aurman -S -c` : 清除包缓存（用两次 `--clean` 参数清除所有包缓存）

## Links
- More information: https://github.com/polygamma/aurman#syntax
