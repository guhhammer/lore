# ansible-lint

Apply rules and follow best practices with your automation content.

## Usage
- `ansible-lint path/to/playbook_file path/to/role_directory` : lint a specific playbook and a role directory
- `ansible-lint -x rule1,rule2,... path/to/playbook_file` : lint a playbook while excluding specific rules
- `ansible-lint -o -p path/to/playbook_file` : lint a playbook in offline mode and format output as PEP8
- `ansible-lint -r path/to/custom_rules_directory path/to/playbook_file` : lint a playbook using a custom rules directory
- `ansible-lint path/to/project_directory` : lint all Ansible content recursively in a given directory

## Categories
files

## Links
- More information: https://docs.ansible.com/projects/lint/
- tldr page: https://tldr.inbrowser.app/pages/common/ansible-lint
