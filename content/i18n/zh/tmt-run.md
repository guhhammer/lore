# tmt run

执行测试步骤。默认情况下，所有测试步骤都被执行。

## Usage
- `tmt run` : 在每一个计划中执行所有测试步骤
- `tmt run discover -v` : 仅在发现步骤中显示将要执行的测试
- `tmt run -a provision -h container -i fedora:rawhide` : 运行所有测试步骤并调整测试环境配置步骤选项
- `tmt run plan -n /plan/name test -n /test/name` : 仅执行选定的计划和测试
- `tmt run -l report -h html -o` : 在网页浏览器中显示上次运行的结果
- `tmt run -c key=value -c distro=fedora` : 在提供的上下文中运行测试
- `tmt run -a execute -h tmt --interactive` : 以交互方式运行测试（在测试运行过程中调试测试代码）
- `tmt run -n -vvv` : 使用干模式查看接下来将发生的动作，并将输出详实度设置为最高级

## Links
- More information: https://tmt.readthedocs.io/en/stable/stories/cli.html#run
