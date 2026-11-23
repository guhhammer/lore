# ansible-doc

Отображать информацию о модулях, установленных в библиотеках Ansible. Выводить краткий список плагинов и их описания.

## Usage
- `ansible-doc -l` : вывести список доступных плагинов действий (модулей)
- `ansible-doc -t become|cache|callback|cliconf|connection|... -l` : вывести список доступных плагинов определённого типа
- `ansible-doc имя_плагина` : показать информацию об определённом плагине действия (модуле)
- `ansible-doc -t become|cache|callback|cliconf|connection|... имя_плагина` : показать информацию о плагине определённого типа
- `ansible-doc -s имя_плагина` : показать фрагмент playbook для плагина действия (модуля)
- `ansible-doc -j имя_плагина` : показать информацию о плагине действия (модуле) в формате JSON

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-doc.html
