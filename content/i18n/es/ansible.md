# ansible

Gestiona grupos de ordenadores remotamente sobre SSH. (usa el archivo `/etc/ansible/hosts` para añadir nuevos grupos/hosts). Algunos subcomandos como `galaxy` tienen su propia documentación de uso.

## Usage
- `ansible grupo --list-hosts` : lista de hosts pertenecientes a un grupo
- `ansible grupo -m ping` : hace ping a un grupo de hosts invocando al módulo ping
- `ansible grupo -m setup` : muestra información sobre un grupo de hosts invocando al módulo setup
- `ansible grupo -m command -a 'mi_comando'` : ejecuta un comando en un grupo de hosts invocando el módulo command con argumentos
- `ansible grupo -b --ask-become-pass -m command -a 'mi_comando'` : ejecuta un comando con privilegios administrativos
- `ansible grupo -i archivo_de_inventario -m command -a 'mi_comando'` : ejecuta un comando utilizando un archivo de inventario personalizado
- `ansible localhost -m debug -a 'var=groups.keys()'` : lista los grupos de un inventario

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
