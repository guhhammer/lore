# emacs

El editor extensible, personalizable, autodocumentado, en tiempo real.

## Usage
- `emacs ruta/al/archivo` : inicia Emacs y abre un archivo
- `emacs +line_number ruta/al/archivo` : abre un archivo en un número de línea especificado
- `emacs --script ruta/al/archivo.el` : ejecuta un archivo Emacs Lisp como guión (script)
- `emacs -nw` : inicia Emacs en modo consola (sin una ventana X)
- `emacs --daemon` : inicia un servidor Emacs en segundo plano (accesible a través de `emacsclient`)
- `emacsclient --eval '(save-buffers-kill-emacs)'` : detiene un servidor Emacs en funcionamiento y todas sus instancias, pidiendo confirmación en archivos no guardados
- `<Ctrl x><Ctrl s>` : guarda un archivo en Emacs
- `<Ctrl x><Ctrl c>` : sale de Emacs

## Links
- More information: https://www.gnu.org/software/emacs
