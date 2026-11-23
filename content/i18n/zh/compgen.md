# compgen

用于在 bash 中自动完成的内置命令，按两次 `<Tab>` 键即可调用该命令。

## Usage
- `compgen -c` : 显示所有可以执行的命令
- `compgen -c 字符串` : 列出所有以指定字符串开头的可执行命令
- `compgen -a` : 列出所有别名
- `compgen -A function` : 列出所有可以运行的函数
- `compgen -k` : 列出所有 shell 的保留关键字
- `compgen -ac ls` : 查看以 'ls' 开头的所有可用命令和别名
- `compgen -u` : 列出系统中所有用户
- `compgen --help` : 显示帮助

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-compgen
