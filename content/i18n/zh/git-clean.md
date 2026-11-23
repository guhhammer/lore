# git clean

从工作区删除未被 Git 跟踪的文件。

## Usage
- `git clean` : 删除未跟踪的文件
- `git clean -i` : 交互式删除未跟踪的文件
- `git clean -n` : 显示将被删除的文件（模拟运行，不实际删除）
- `git clean -f` : 强制删除未跟踪的文件
- `git clean -f -d` : 强制删除未跟踪的目录
- `git clean -x` : 删除未跟踪的文件（包括被忽略的文件，在 `.gitignore` 和 `.git/info/exclude`的文件）

## Links
- More information: https://git-scm.com/docs/git-clean
