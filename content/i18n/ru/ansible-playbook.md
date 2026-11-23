# ansible-playbook

Выполнять задачи, определённые в playbook, на удалённых машинах по SSH.

## Usage
- `ansible-playbook playbook` : запустить задачи из playbook
- `ansible-playbook playbook -i файл_инвентаря` : запустить задачи из playbook с пользовательским инвентарём хостов
- `ansible-playbook playbook -e "переменная1=значение1 переменная2=значение2"` : запустить задачи из playbook с дополнительными переменными, определёнными в командной строке
- `ansible-playbook playbook -e "@переменные.json"` : запустить задачи из playbook с дополнительными переменными, определёнными в JSON-файле
- `ansible-playbook playbook -t тег1,тег2` : запустить задачи из playbook для указанных тегов
- `ansible-playbook playbook --start-at-task имя_задачи` : запустить задачи из playbook, начиная с определённой задачи
- `ansible-playbook playbook -C -D` : симулировать выполнение задач из playbook без внесения изменений

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
