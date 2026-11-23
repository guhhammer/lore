# git gui

Una GUI para Git para gestionar ramas, remotos, confirmaciones de cambio y realizar fusiones locales.

## Usage
- `git gui` : inicia la GUI
- `git gui blame ruta/al/archivo` : muestra un archivo específico con el nombre del autor y el hash de confirmación en cada línea
- `git gui blame revisión ruta/al/archivo` : abre `git gui blame` en una revisión específica
- `git gui blame --line=línea ruta/al/archivo` : abre `git gui blame` y desplaza la vista para centrarla en una línea específica
- `git gui citool` : abre una ventana para hacer una confirmación y vuelve al intérprete de comandos cuando se haya completado
- `git gui citool --amend` : abre `git gui citool` en modo "Modificar la última confirmación"
- `git gui citool --nocommit` : abre `git gui citool` en modo de solo lectura
- `git gui browser maint` : muestra un navegador para el árbol de una rama específica, abriendo la herramienta de autoría al hacer clic en los archivos

## Links
- More information: https://git-scm.com/docs/git-gui
