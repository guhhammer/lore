# gt

Crea y gestiona secuencias de cambios de código dependientes (stacks) para Git y GitHub.

## Usage
- `gt init` : inicializa `gt` para el repositorio en el directorio actual
- `gt create nombre_de_rama` : crea una nueva rama apilada sobre la rama actual y confirmar los cambios por etapas
- `gt modify -cam mensaje_de_commit` : crea un nuevo commit y arreglar las ramas apiladas
- `gt stack submit` : fuerza el push de todas las ramas de la pila actual a GitHub y crea o actualiza PRs
- `gt co nombre_rama` : comprueba una rama diferente (solicita el modo interactivo cuando se omite el nombre de la rama)
- `gt sync` : sincroniza la pila con la versión remota (también elimina las ramas fusionadas)
- `gt log short` : registra todas las pilas rastreadas
- `gt subcomando --help` : muestra la ayuda de un subcomando específico

## Links
- More information: https://graphite.com/docs/get-started
