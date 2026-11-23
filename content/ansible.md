# ansible

Manage groups of computers remotely over SSH. (use the `/etc/ansible/hosts` file to add new groups/hosts). Some subcommands such as `galaxy` have their own usage documentation.

## Usage
- `ansible group --list-hosts` : list hosts belonging to a group
- `ansible group -m ping` : ping a group of hosts by invoking the ping module
- `ansible group -m setup` : display facts about a group of hosts by invoking the setup module
- `ansible group -m command -a 'my_command'` : execute a command on a group of hosts by invoking command module with arguments
- `ansible group -b --ask-become-pass -m command -a 'my_command'` : execute a command with administrative privileges
- `ansible group -i inventory_file -m command -a 'my_command'` : execute a command using a custom inventory file
- `ansible localhost -m debug -a 'var=groups.keys()'` : list the groups in an inventory

## Categories
users, network, development

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
- tldr page: https://tldr.inbrowser.app/pages/common/ansible
