# fc

打开最近的命令进行编辑，然后运行它。

## Usage
- `fc` : 在默认系统编辑器中打开最后一个命令，并在编辑后运行
- `fc -e 'emacs'` : 指定一个编辑器打开
- `fc -l` : 从历史记录中列出最近的命令
- `fc -l -r` : 以相反的顺序列出最近的命令
- `fc 编号` : 从历史记录中编辑并运行一个命令
- `fc '416' '420'` : 编辑并运行指定区间内的命令
- `fc --help` : 显示帮助

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-fc
