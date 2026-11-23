# ansible-galaxy

执行与 Ansible 角色和集合相关的各种操作。

## Usage
- `ansible-galaxy role|collection list` : 列出已安装的角色或集合
- `ansible-galaxy role search 关键字 -vvvvvv` : 使用不同的详细等级搜索角色（`-v` 应该放在最后）
- `ansible-galaxy role install|remove 角色名称1 角色名称2 ...` : 安装或移除角色
- `ansible-galaxy role init 角色名称` : 创建一个新角色
- `ansible-galaxy role info 角色名称` : 获取关于角色的信息
- `ansible-galaxy collection install|remove 集合名称1 集合名称2 ...` : 安装或移除集合
- `ansible-galaxy role|collection -h` : 显示关于角色或集合的帮助信息

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-galaxy.html
