# git add

将更改的文件添加到索引（暂存区）。

## Usage
- `git add 路径/到/文件` : 添加单个文件到索引
- `git add -A` : 添加所有文件（包括已跟踪的和未跟踪的）
- `git add .` : 从当前文件夹开始递归地添加所有文件
- `git add -u` : 只添加已跟踪的文件
- `git add -f 路径/到/文件` : 忽略的文件也添加
- `git add -p` : 交互式挑选部分修改并暂存
- `git add -p 路径/到/文件` : 交互式挑选指定文件的部分修改并暂存
- `git add -i` : 在交互模式下暂存文件

## Links
- More information: https://git-scm.com/docs/git-add
