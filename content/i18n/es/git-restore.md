# git restore

Restaura los archivos del árbol de trabajo. Requiere la version 2.23+ de Git.

## Usage
- `git restore ruta/al/archivo` : restaura un archivo sin marcar a la versión de la confirmación actual (HEAD)
- `git restore -s confirmación ruta/al/archivo` : restaura un archivo sin marcar a la versión de una confirmación específica
- `git restore :/` : descarta los cambios sin confirmación para los archivos rastreados
- `git restore -S ruta/al/archivo` : desmarca un archivo
- `git restore -S :/` : desmarca todos los archivos
- `git restore -W -S :/` : descarta todos los cambios de los archivos, marcados o no
- `git restore -p` : selecciona interactivamente secciones de archivos para restaurar

## Links
- More information: https://git-scm.com/docs/git-restore
