# git commit-tree

用于创建提交对象的底层工具。

## Usage
- `git commit-tree 树对象哈希 -m "提交信息"` : 创建带有指定提交信息的提交对象
- `git commit-tree 树对象哈希 -F 路径/到/文件` : 从文件读取提交信息创建提交对象（使用 `-` 表示从标准输入读取）
- `git commit-tree 树对象哈希 -m "提交信息" -S` : 创建 GPG 签名过的提交对象
- `git commit-tree 树对象哈希 -m "提交信息" -p 父提交哈希` : 创建指定父提交的提交对象

## Links
- More information: https://git-scm.com/docs/git-commit-tree
