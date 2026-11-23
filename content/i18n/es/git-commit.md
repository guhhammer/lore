# git commit

Realiza confirmaciones de los archivos al repositorio.

## Usage
- `git commit -m "mensaje"` : realiza una confirmación de los archivos marcados al repositorio con un mensaje
- `git commit -F ruta/al/archivo_con_mensaje_de_la_confirmación` : realiza una confirmación de los archivos marcados con un mensaje leído desde un archivo
- `git commit -a -m "mensaje"` : marca automáticamente todos los archivos modificados y realiza una confirmación con un mensaje
- `git commit -S identificador_de_llave -m "mensaje"` : confirma todos los archivos preparados y los firma con una llave de GPG (o la llave en el archivo de configuración si no se especifica un argumento)
- `git commit --amend` : sustituye la última confirmación con los cambios marcados actualmente, cambiando el hash de la confirmación
- `git commit ruta/al/archivo1 ruta/al/archivo2 ...` : realiza una confirmación para archivos específicos (marcados previamente)
- `git commit -m "mensaje" --allow-empty` : crea una confirmación, incluso si no hay archivos marcados

## Links
- More information: https://git-scm.com/docs/git-commit
