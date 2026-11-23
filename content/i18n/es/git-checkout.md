# git checkout

Comprueba una rama o rutas con el árbol de trabajo.

## Usage
- `git checkout -b nombre_de_la_rama` : crea una nueva rama y se cambia a la misma
- `git checkout -b nombre_de_la_rama referencia` : crea una nueva rama a partir de una referencia específica (rama, remoto/rama, las etiquetas son ejemplos de referencias válidas) y cambiarse a esta
- `git checkout nombre_de_la_rama` : cambia a una rama local existente
- `git checkout -` : cambia a la rama previamente comprobada
- `git checkout -t nombre_remoto/nombre_de_la_rama` : cambia a una rama remota existente
- `git checkout .` : descarta todos los cambios sin marcar en el directorio actual (vea `git reset` para más comandos para deshacer)
- `git checkout ruta/al/archivo` : descarta los cambios no marcados de un archivo específico
- `git checkout nombre_de_la_rama -- ruta/al/archivo` : reemplaza un archivo en el directorio actual con la versión de éste en la confirmación de una rama específica

## Links
- More information: https://git-scm.com/docs/git-checkout
