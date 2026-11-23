# disown

允许子进程在其所属的 shell 结束后继续运行。

## Usage
- `disown` : 解除当前作业与 shell 的关联
- `disown %作业编号` : 解除指定作业与 shell 的关联（运行 `jobs` 查找作业编号）
- `disown -a` : 解除所有作业与 shell 的关联（仅限 Bash）
- `disown -h %作业编号` : 保留作业（不解除关联），但标记为 shell 退出时不再接收 `SIGHUP`（仅限 Bash）

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-disown
