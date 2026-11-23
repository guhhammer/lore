# ansible-galaxy

Perform various Ansible Role and Collection related operations.

## Usage
- `ansible-galaxy role|collection list` : list installed roles or collections
- `ansible-galaxy role search keyword -vvvvvv` : search for a role with various levels of verbosely (`-v` should be specified at the end)
- `ansible-galaxy role install|remove role_name1 role_name2 ...` : install or remove role(s)
- `ansible-galaxy role init role_name` : create a new role
- `ansible-galaxy role info role_name` : get information about a role
- `ansible-galaxy collection install|remove collection_name1 collection_name2 ...` : install or remove collection(s)
- `ansible-galaxy role|collection -h` : display help about roles or collections

## Categories
packages

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-galaxy.html
- tldr page: https://tldr.inbrowser.app/pages/common/ansible-galaxy
