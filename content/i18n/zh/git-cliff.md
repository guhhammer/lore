# git cliff

高度可定制的变更日志生成工具。

## Usage
- `git cliff >CHANGELOG.md` : 从 Git 仓库所有提交生成变更日志并保存到 `CHANGELOG.md`
- `git cliff -l` : 从最新标签之后的提交生成变更日志并输出到标准输出
- `git cliff --current` : 为当前标签对应的提交生成变更日志（需先使用 `git checkout` 检出到一个标签）
- `git cliff -u` : 为尚未打标签的提交生成变更日志
- `git cliff -i` : 在当前目录生成默认配置文件 `cliff.toml`

## Links
- More information: https://git-cliff.org/docs/usage/args/
