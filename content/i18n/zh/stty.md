# stty

设置终端设备接口的选项。

## Usage
- `stty -a` : 显示当前终端的所有设置
- `stty rows|cols 数量` : 设置行数或列数
- `stty -F 路径/到/设备文件 speed` : 获取设备的实际传输速度
- `stty sane` : 重置所有模式为当前终端的合理默认值
- `stty raw|cooked` : 在原始模式和普通模式之间切换
- `stty -echo|echo` : 关闭或开启字符回显
- `stty --help` : 显示帮助

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stty-invocation.html
