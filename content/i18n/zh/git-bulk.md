# git bulk

批量操作多个 Git 仓库。 属于 `git-extras`的一部分。

## Usage
- `git bulk --addcurrent 工作区名称` : 将当前目录注册为工作区
- `git bulk --addworkspace 工作区名称 到仓库的绝对路径` : 注册一个工作区用于批量操作
- `git bulk --addworkspace 工作区名称 仓库父目录的绝对路径 --from 远程仓库地址` : 在指定目录克隆仓库并注册为工作区
- `git bulk --addworkspace 工作区名称 /路径/到/根/目录 --from /路径/到/文件` : 从换行分隔的远程仓库列表克隆并注册为工作区
- `git bulk --listall` : 列出所有已注册的工作区
- `git bulk 命令 命令参数` : 在当前工作区的所有仓库上执行 Git 命令
- `git bulk --removeworkspace 工作区名称` : 删除指定工作区
- `git bulk --purge` : 删除所有工作区

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-bulk
