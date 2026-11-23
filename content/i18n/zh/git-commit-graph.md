# git commit-graph

生成和验证 Git 提交图文件。

## Usage
- `git commit-graph write` : 为仓库本地 `.git` 目录中的打包提交生成提交图文件
- `git show-ref -s | git commit-graph write --stdin-commits` : 生成包含所有可达提交的提交图文件
- `git rev-parse HEAD | git commit-graph write --stdin-commits --append` : 生成包含当前提交图文件中所有提交以及从 `HEAD` 可达提交的提交图文件（追加模式）

## Links
- More information: https://git-scm.com/docs/git-commit-graph
