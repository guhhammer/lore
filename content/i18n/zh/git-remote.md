# git remote

管理跟踪的远程仓库（remotes）。

## Usage
- `git remote -v` : 列出已经存在的远程仓库，包括它们的名字和 URL
- `git remote show 远程仓库名字` : 查看某个远程仓库的信息
- `git remote add 远程仓库名字 远程仓库url` : 添加远程仓库
- `git remote set-url 远程仓库名字 新url` : 更改远程仓库地址链接（使用 `--add` 选项不会移除现有的 URL）
- `git remote get-url 远程仓库名字` : 查看远程仓库的 URL
- `git remote remove 远程仓库名字` : 移除远程仓库
- `git remote rename 旧名字 新名字` : 重命名远程仓库

## Links
- More information: https://git-scm.com/docs/git-remote
