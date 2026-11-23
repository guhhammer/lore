# ansible-galaxy

Realiza varias operaciones de Ansible Role y algunas relacionadas a Collection.

## Usage
- `ansible-galaxy role|collection list` : enumera roles instalados o colecciones
- `ansible-galaxy role search keyword -vvvvvv` : busca un rol con varios niveles de verbosidad (`-v` debe ser específicado al final)
- `ansible-galaxy role install|remove rol_nombre1 rol_nombre2 ...` : instala o elimina roles
- `ansible-galaxy role init rol_nombre` : crea un nuevo rol
- `ansible-galaxy role info rol_nombre` : obtiene información acerca de un rol
- `ansible-galaxy collection install|remove colección_nombre1 colección_nombre2 ...` : instala o elimina colecciones
- `ansible-galaxy role|collection -h` : despliega más información (ayuda) acerca de roles o colecciones

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-galaxy.html
