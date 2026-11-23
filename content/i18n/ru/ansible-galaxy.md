# ansible-galaxy

Выполнять различные операции с ролями и коллекциями Ansible.

## Usage
- `ansible-galaxy role|collection list` : вывести список установленных ролей или коллекций
- `ansible-galaxy role search ключевое_слово -vvvvvv` : найти роль с различными уровнями подробности (`-v` указывается в конце)
- `ansible-galaxy role install|remove имя_роли1 имя_роли2 ...` : установить или удалить роль(и)
- `ansible-galaxy role init имя_роли` : создать новую роль
- `ansible-galaxy role info имя_роли` : получить информацию о роли
- `ansible-galaxy collection install|remove имя_коллекции1 имя_коллекции2 ...` : установить или удалить коллекцию(и)
- `ansible-galaxy role|collection -h` : показать справку о ролях или коллекциях

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-galaxy.html
