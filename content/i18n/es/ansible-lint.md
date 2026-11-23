# ansible-lint

Aplica reglas y sigue las mejores prácticas en la automatización de tu contenido.

## Usage
- `ansible-lint ruta/al/playbook ruta/al/directorio_de_roles` : analiza un playbook (archivo de tareas) específico y un directorio de roles con Lint
- `ansible-lint -x regla1, regla2,... ruta/al/archivo_playbook` : analiza un playbook mientras se excluyen reglas específicas
- `ansible-lint -o -p ruta/al/archivo_playbook` : analiza un playbook en modo sin conexión y le da un formato de salida PEP8
- `ansible-lint -r ruta/al/directorio_de_reglas_personalizadas ruta/al/archivo_playbook` : analiza un playbook usando un directorio de reglas personalizadas
- `ansible-lint ruta/al/directorio_del_proyecto` : analiza todo el contenido Ansible de manera recursiva en un directorio dado

## Links
- More information: https://docs.ansible.com/projects/lint/
