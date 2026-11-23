# fossil commit

Envía archivos a un repositorio Fossil.

## Usage
- `fossil ci` : crea una nueva versión que contiene todos los cambios en el checkout actual; se solicitará al usuario un comentario
- `fossil ci -m "comentario"` : crea una versión que contiene todos los cambios en el checkout actual, utilizando el comentario especificado
- `fossil ci -M ruta/al/archivo_con_comentario` : crea una versión que contiene todos los cambios en el checkout actual con un comentario leído de un archivo específico
- `fossil ci ruta/al/archivo1 ruta/al/archivo2 ...` : crea una versión que contiene cambios de los archivos especificados; se solicitará al usuario un comentario

## Links
- More information: https://fossil-scm.org/home/help/commit
