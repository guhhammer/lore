# gh issue

管理 GitHub issue。

## Usage
- `gh issue view issue_号` : 显示指定的 issue
- `gh issue view issue_号 -w` : 在默认网页浏览器中查看指定的 issue
- `gh issue new -w` : 在默认网页浏览器中创建一个新 issue
- `gh issue ls -L 10 -l "bug"` : 列出最近 10 个带有 "bug" 标签的 issue
- `gh issue ls -s closed -A 用户名` : 列出由指定用户创建的已关闭的 issue
- `gh issue status -R 拥有者/仓库` : 在指定仓库中，显示与当前用户相关的 issue 的状态
- `gh issue reopen issue_号` : 重新打开指定的 issue

## Links
- More information: https://cli.github.com/manual/gh_issue
