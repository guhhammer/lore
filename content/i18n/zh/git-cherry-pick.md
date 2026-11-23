# git cherry-pick

将现有提交的变更应用到当前分支。 如需应用到其他分支，请先用 `git checkout` 切换到目标分支。

## Usage
- `git cherry-pick 提交哈希` : 将单个提交应用到当前分支
- `git cherry-pick 起始提交~..结束提交` : 将连续多个提交应用到当前分支（也可参考 `git rebase --onto`）
- `git cherry-pick 提交1 提交2 ...` : 将多个（非连续的）提交应用到当前分支
- `git cherry-pick -n 提交哈希` : 将提交变更应用到工作区但不自动创建提交

## Links
- More information: https://git-scm.com/docs/git-cherry-pick
