# ansible-playbook

Execute tasks defined in playbook on remote machines over SSH.

## Usage
- `ansible-playbook playbook` : run tasks in playbook
- `ansible-playbook playbook -i inventory_file` : run tasks in playbook with custom host inventory
- `ansible-playbook playbook -e "variable1=value1 variable2=value2"` : run tasks in playbook with extra variables defined via the command-line
- `ansible-playbook playbook -e "@variables.json"` : run tasks in playbook with extra variables defined in a JSON file
- `ansible-playbook playbook -t tag1,tag2` : run tasks in playbook for the given tags
- `ansible-playbook playbook --start-at-task task_name` : run tasks in a playbook starting at a specific task
- `ansible-playbook playbook -C -D` : simulate running tasks in a playbook without making any changes

## Categories
network, text

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
- tldr page: https://tldr.inbrowser.app/pages/common/ansible-playbook
