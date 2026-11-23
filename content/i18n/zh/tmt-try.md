# tmt try

测试及环境快速上手。

## Usage
- `tmt try` : 快速尝试默认的测试环境配置方法（当前工作目录中没有测试）
- `cd path/to/test && tmt try` : 在当前的工作目录中运行一个测试
- `tmt try fedora-41` : 使用特定的操作系统
- `tmt try fedora@container` : 选择定制的镜像和测试环境配置方法
- `tmt try -t feature` : 根据定制的筛选条件选择测试
- `tmt try -a` : 配置客户机并等待用户输入指令
- `tmt try -l` : 直接登录到客户机
- `tmt try --help` : 显示帮助

## Links
- More information: https://tmt.readthedocs.io/en/stable/stories/cli.html#try
