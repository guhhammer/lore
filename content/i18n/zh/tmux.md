# tmux

终端复用器。 支持包含窗口、窗格等的多个会话。

## Usage
- `tmux` : 启动一个新会话
- `tmux new -s name` : 启动一个指定名称的新会话（session）
- `tmux ls` : 列出现有会话
- `tmux a` : 连接到最近使用的会话
- `<Ctrl b><d>` : 从当前 tmux 会话断开连接
- `<Ctrl b><c>` : 创建一个新窗口（在 tmux 会话中）
- `<Ctrl b><w>` : 在会话和窗口之间切换（在 tmux 会话中）
- `tmux kill-session -t name` : 终止指定目标会话（target）

## Links
- More information: https://github.com/tmux/tmux
