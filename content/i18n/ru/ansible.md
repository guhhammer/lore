# ansible

Управлять группами компьютеров удалённо по SSH. (Используйте файл `/etc/ansible/hosts` для добавления новых групп/хостов). Некоторые подкоманды, такие как `galaxy`, имеют собственную документацию по использованию.

## Usage
- `ansible группа --list-hosts` : вывести список хостов, принадлежащих группе
- `ansible группа -m ping` : проверить доступность (ping) группы хостов, вызвав модуль ping
- `ansible группа -m setup` : вывести факты о группе хостов, вызвав модуль setup
- `ansible группа -m command -a 'команда'` : выполнить команду для группы хостов, вызвав модуль command с аргументами
- `ansible группа -b --ask-become-pass -m command -a 'команда'` : выполнить команду с правами администратора
- `ansible группа -i файл_инвентаря -m command -a 'команда'` : выполнить команду, используя указанный файл инвентаря
- `ansible localhost -m debug -a 'var=groups.keys()'` : вывести список групп в инвентаре

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
