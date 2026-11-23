# ansible-lint

对自动化内容应用规则并遵循最佳实践。

## Usage
- `ansible-lint 路径/到/playbook_文件 路径/到/角色_目录` : 对指定的 playbook 和角色目录执行 lint 检查
- `ansible-lint -x 规则1,规则2,... 路径/到/playbook_文件` : 在排除特定规则的情况下对 playbook 执行 lint 检查
- `ansible-lint -o -p 路径/到/playbook_文件` : 以离线模式对 playbook 执行 lint 检查，并将输出格式化为 PEP8 风格
- `ansible-lint -r 路径/到/自定义_规则_目录 路径/到/playbook_文件` : 使用自定义规则目录对 playbook 执行 lint 检查
- `ansible-lint 路径/到/项目_目录` : 递归地对给定目录中的所有 Ansible 内容执行 lint 检查

## Links
- More information: https://docs.ansible.com/projects/lint/
