# dmenu

动态菜单。 根据文本输入创建菜单，其中每一项都在新行中。

## Usage
- `ls | dmenu` : 显示 `ls` 命令输出的菜单
- `echo -e "red\ngreen\nblue" | dmenu` : 显示包含自定义项目的菜单，并用新行（`\n`）分隔
- `echo -e "red\ngreen\nblue" | dmenu >color.txt` : 让用户在多个项目之间进行选择，然后将所选项目保存到文件中
- `ls | dmenu -m 1` : 在特定的监视器上启动 `dmenu`
- `ls | dmenu -b` : 在屏幕底部显示 `dmenu`

## Links
- More information: https://manned.org/dmenu
