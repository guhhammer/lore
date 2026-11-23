# qwen

使用 Qwen3-Coder 启动一个交互式提示。

## Usage
- `qwen` : 启动一个 REPL 会话以进行交互式对话
- `echo "总结罗马的历史" | qwen -p` : 将另一个命令的输出发送给 Qwen 并立即退出
- `qwen -m model_name` : 覆盖默认模型（默认值：qwen3-coder-max）
- `qwen -s` : 在沙盒容器中运行
- `qwen -i "给我一个 Python 中递归的示例"` : 执行一个提示，然后保持在交互模式中
- `qwen -a` : 在上下文中包含所有文件
- `qwen --show-memory-usage` : 在状态栏中显示内存使用情况

## Links
- More information: https://github.com/QwenLM/qwen-code
