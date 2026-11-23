# git init

初始化新的本地 Git 仓库。

## Usage
- `git init` : 初始化新的本地仓库
- `git init -b 分支名` : 初始化仓库，并指定初始分支的名字（通常默认为 `master` 或 `main`）
- `git init --object-format sha256` : 初始化仓库，使用 SHA256 算法生成对象哈希（需 Git 2.29+ 版本支持）
- `git init --bare` : 创建裸仓库，适合用作 SSH 远程仓库

## Links
- More information: https://git-scm.com/docs/git-init
