# emacsclient

Abre archivos en un servidor Emacs existente.

## Usage
- `emacsclient ruta/al/archivo` : abre un archivo en un servidor Emacs existente (utilizando GUI si está disponible)
- `emacsclient -nw ruta/al/archivo` : abre un archivo en modo consola (sin una ventana X)
- `emacsclient -c ruta/al/archivo` : abre un archivo en una nueva ventana Emacs
- `emacsclient -e '(comando)'` : evalúa un comando, imprime la salida a `stdout`, y luego sale
- `emacsclient -a editor ruta/al/archivo` : especifica un editor alternativo en caso de que ningún servidor Emacs esté funcionando
- `emacsclient -e '(save-buffers-kill-emacs)'` : detiene un servidor Emacs en funcionamiento y todas sus instancias, pidiendo confirmación en archivos no guardados
