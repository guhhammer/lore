# git switch

切换 Git 分支。要求 Git 版本在 2.23 以上。

## Usage
- `git switch 分支名字` : 切换到一个已有的分支
- `git switch -c 分支名字` : 创建并切换到一个新分支
- `git switch -c 分支名字 指定提交` : 创建并切换到基于某个提交的新分支
- `git switch -` : 切换到之前的分支
- `git switch --recurse-submodules 分支名字` : 切换到一个分支，并更新所有匹配的子模块
- `git switch -m 分支名字` : 切换到一个分支，并和当前分支以及暂未提交的修改进行三方合并

## Links
- More information: https://git-scm.com/docs/git-switch
