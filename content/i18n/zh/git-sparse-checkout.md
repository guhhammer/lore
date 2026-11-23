# git sparse-checkout

仅检出仓库的部分文件，而不是克隆或检出全部文件（稀疏检出）。

## Usage
- `git sparse-checkout init` : 启用稀疏检出
- `git sparse-checkout disable` : 禁用稀疏检出并恢复完整仓库
- `git sparse-checkout set 路径/到/目录` : 指定要包含的目录（或文件）
- `git sparse-checkout add 路径/到/目录` : 稍后添加更多

## Links
- More information: https://manned.org/git-sparse-checkout
