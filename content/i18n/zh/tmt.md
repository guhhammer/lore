# tmt

创建、运行和调试测试的测试管理工具。 诸如`运行`、`尝试`等子命令，均有相应的用法文档。

## Usage
- `tmt` : 列举可用的测试、计划和用户故事
- `tmt init` : 初始化测试管理工具的文件/项目结构
- `tmt test create -t beakerlib --link verifies:issue#1234` : 基于模板和链接创建新的测试
- `tmt test|plan|story ls pattern` : 列出可用的测试、计划和用户故事
- `tmt -c arch=aarch64 test show` : 在给定的上下文中显示详细的测试元数据
- `tmt lint` : 根据说明书验证测试管理工具文件的有效性
- `tmt tests ls -f tag:foo -f tier:0` : 使用筛选条件
- `tmt --help` : 显示帮助

## Links
- More information: https://tmt.readthedocs.io/en/stable/examples.html
