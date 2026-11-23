# gh run

查看、运行和监控最近的 GitHub Actions 工作流运行记录。

## Usage
- `gh run view` : 以交互式方式选择运行记录以查看其任务信息
- `gh run view 工作流运行编号` : 显示指定运行记录的信息
- `gh run view -j 任务编号` : 显示指定任务的各个步骤的信息
- `gh run view -j 任务编号 --log` : 显示指定任务的日志
- `gh run view 工作流运行编号 --exit-status && echo "运行中或已通过"` : 检查指定的工作流运行，如果运行失败则退出并返回非零状态码
- `gh run watch` : 以交互式方式选择一个正在进行的运行记录，并等待其运行完毕
- `gh run watch 工作流运行编号` : 显示某个运行记录的任务并等待其运行完毕
- `gh run rerun 工作流运行编号` : 重新运行指定的工作流

## Links
- More information: https://cli.github.com/manual/gh_run
