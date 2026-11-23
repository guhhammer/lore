# fish

The Friendly Interactive SHell, 一个设计为用户友好的命令行解释器。

## Usage
- `fish` : 启动交互式 shell 会话
- `fish -N` : 启动不加载启动配置的交互式 shell 会话
- `fish -c "echo 'fish is executed'"` : 执行特定命令
- `fish 路径/到/脚本.fish` : 执行特定脚本
- `fish -n 路径/到/脚本.fish` : 检查特定脚本是否有语法错误
- `echo "echo 'fish is executed'" | fish` : 从 `stdin` 执行特定命令
- `fish -P` : 在专用模式下启动交互式 shell 会话，其中 shell 不会访问旧历史记录或保存新历史记录
- `set -U -x 变量名 变量值` : 定义并导出一个在 shell 重启后持续存在的环境变量（内置）

## Links
- More information: https://fishshell.com/docs/current/cmds/fish.html
