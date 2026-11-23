# ansible-doc

Muestra información sobre los módulos instalados en las bibliotecas de Ansible. Muestra una concisa lista de complementos y sus breves descripciones.

## Usage
- `ansible-doc -l` : lista de complementos disponibles acorde a su acción (módulos)
- `ansible-doc -t become|cache|callback|cliconf|connection|... -l` : lista de complementos disponibles dado un tipo específico
- `ansible-doc nombre_complemento` : muestra información sobre un complemento acorde a su acción específica (módulo)
- `ansible-doc -t become|cache|callback|cliconf|connection|... nombre_complemento` : muestra información acerca de un complemento dado un tipo específico
- `ansible-doc -s nombre_complemento` : muestra fragmentos de las acciones respecto al tipo de complemento y su especificidad de tipo de acción (módulos)
- `ansible-doc -j nombre_complemento` : muestra información de acuerdo al complemento dada su especificidad de acción (módulo) como JSON

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-doc.html
