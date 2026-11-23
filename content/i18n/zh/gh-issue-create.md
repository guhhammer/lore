# gh issue create

创建 GitHub 仓库中的 issue。

## Usage
- `gh issue new` : 以交互式方式在当前仓库中创建一个新 issue
- `gh issue new -l "bug"` : 以交互式方式创建一个带有 "bug" 标签的新 issue
- `gh issue new -a 用户1,用户2,...` : 以交互式方式创建一个新 issue 并分配给指定的用户
- `gh issue new -t "标题" -b "描述" -a "@me"` : 创建一个具有标题和描述的新 issue，并分配给当前用户
- `gh issue new -F 路径/到/文件` : 以交互式方式创建一个新 issue，并从文件中读取其描述正文
- `gh issue new -w` : 在默认网页浏览器中创建一个新 issue
- `gh issue new -h` : 显示帮助

## Links
- More information: https://cli.github.com/manual/gh_issue_create
