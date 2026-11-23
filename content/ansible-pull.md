# ansible-pull

Pull ansible playbooks from a VCS repo and executes them for the local host.

## Usage
- `ansible-pull -U repository_url` : pull a playbook from a VCS and execute a default `local.yml` playbook
- `ansible-pull -U repository_url playbook` : pull a playbook from a VCS and execute a specific playbook
- `ansible-pull -U repository_url -C branch playbook` : pull a playbook from a VCS at a specific branch and execute a specific playbook
- `ansible-pull -U repository_url -i hosts_file playbook` : pull a playbook from a VCS, specify hosts file, and execute a specific playbook

## Categories
network

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html
- tldr page: https://tldr.inbrowser.app/pages/common/ansible-pull
