# git annex

用 Git 管理文件，但不将其内容提交到版本库中。 当文件被附属时，其内容会被移至键值存储中，并创建一个指向该内容的符号链接。

## Usage
- `git annex init` : 初始化一个带有 Git annex 的仓库
- `git annex add 路径/到/文件或目录` : 添加文件
- `git annex status 路径/到/文件或目录` : 显示文件或目录的当前状态
- `git annex 远程仓库名` : 同步本地仓库与远程仓库
- `git annex get 路径/到/文件或目录` : 获取文件或目录（从键值存储中恢复内容）
- `git annex help` : 显示帮助

## Links
- More information: https://git-annex.branchable.com/git-annex/
