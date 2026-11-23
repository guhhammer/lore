# alias

创建别名——用给定的字符串指代特定的命令。 别名只会在当前的 shell 会话中生效，除非它们在 shell 的配置文件中被定义，例如`~/.bashrc`。

## Usage
- `alias` : 列出所有别名
- `alias 别名="命令"` : 创建一个通用的别名
- `alias 别名` : 通过给定的别名查看它所指代的命令
- `unalias 别名` : 移除一个别名
- `alias rm="rm --interactive"` : 将 `rm` 转换为交互式命令
- `alias la="ls --all"` : 创建别名 `la` 来指代 `ls --all`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
