# ansible-playbook

通过 SSH 协议在远程计算机上执行 playbook 中定义的任务。

## Usage
- `ansible-playbook playbook` : 执行 playbook 中的任务
- `ansible-playbook playbook -i 清单文件` : 使用自定义主机清单执行 playbook 中的任务
- `ansible-playbook playbook -e "变量1=值1 变量2=值2"` : 使用通过命令行定义的额外变量执行 playbook 中的任务
- `ansible-playbook playbook -e "@变量.json"` : 使用在 JSON 文件中定义的额外变量执行 playbook 中的任务
- `ansible-playbook playbook -t 标签1,标签2` : 执行 playbook 中的指定标签的任务
- `ansible-playbook playbook --start-at-task 任务名称` : 从指定任务开始执行 playbook 中的任务
- `ansible-playbook playbook -C -D` : 以不做任何更改（试执行）方式执行 playbook 中的任务

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
