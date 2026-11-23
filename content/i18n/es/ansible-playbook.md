# ansible-playbook

Ejecuta tareas definidas en un playbook (archivo de tareas) en máquinas remotas sobre SSH.

## Usage
- `ansible-playbook playbook` : ejecuta tareas en el playbook dado
- `ansible-playbook playbook -i inventory_file` : ejecuta tareas en el playbook dado con inventario de host personalizado
- `ansible-playbook playbook -e "variable1=valor1 variable2=valor2"` : ejecuta tareas en el playbook dado con variables extra definidas con la línea de comandos
- `ansible-playbook playbook -e "@variables.json"` : ejecuta tareas en el playbook dado con variables extra definidas en un archivo JSON
- `ansible-playbook playbook -t etiqueta1,etiqueta2` : ejecuta tareas en el playbook dado con etiquetas específicas
- `ansible-playbook playbook --start-at-task nombre_tarea` : ejecuta tareas en el playbook dado empezando por una tarea determinada
- `ansible-playbook playbook -C -D` : ejecuta tareas en el playbook dado sin realizar ningún cambio (dry-run)

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
