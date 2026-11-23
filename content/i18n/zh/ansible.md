# ansible

通过 SSH 协议远程管理计算机组。使用 `/etc/ansible/hosts` 文件来添加组 / 主机。

## Usage
- `ansible 组 --list-hosts` : 列出给定组下的所有主机
- `ansible 组 -m ping` : 调用 ping 模块来 ping 一组主机
- `ansible 组 -m setup` : 通过调用安装模块来显示关于一组主机的信息
- `ansible 组 -m command -a '命令'` : 调用命令模块并使用给定的参数来对一组主机执行命令
- `ansible 组 -b --ask-become-pass -m command -a '命令'` : 以管理员权限执行一个命令
- `ansible 组 -i 清单文件 -m command -a '命令'` : 使用自定义的清单文件执行一个命令
- `ansible localhost -m debug -a 'var=groups.keys()'` : 列出清单中的组

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
