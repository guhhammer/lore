# portageq

Consulta información sobre Portage, el gestor de paquetes de Gentoo Linux. Las variables de entorno específicas de Portage que se pueden consultar están listadas en `/var/db/repos/gentoo/profiles/info_vars`.

## Usage
- `portageq envvar variable` : muestra el valor de una variable de entorno específica de Portage
- `portageq repos_config /` : muestra una lista detallada de los repositorios configurados con Portage
- `portageq get_repos /` : muestra una lista de repositorios ordenados por prioridad (el más alto, primero)
- `portageq metadata / ebuild|porttree|binary|... categoría/paquete BDEPEND|DEFINED_PHASES|DEPEND|...` : muestra un fragmento específico de metadatos sobre un átomo (por ejemplo, el nombre del paquete incluyendo la versión)

## Links
- More information: https://wiki.gentoo.org/wiki/Portageq
