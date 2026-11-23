# fzf

Buscador aproximado (fuzzy search) de la línea de comando.

## Usage
- `find ruta/al/directorio -type f | fzf` : aplica `fzf` a todos los archivos en el directorio especificado
- `ps aux | fzf` : aplica `fzf` a los procesos en ejecución
- `find ruta/al/directorio -type f | fzf -m >ruta/al/archivo` : selecciona varios archivos con `<Shift Tab>` y los escribe a un archivo
- `fzf -q "consulta"` : aplica `fzf` con una consulta especificada
- `fzf -q "^core go$ | rb$ | py$"` : aplica `fzf` en las entradas que comienzan con `core` y finalizan con `go`, `rb`, o `py`
- `fzf -q '!pyc travis'` : aplica `fzf` en entradas que no coinciden con `pyc` y coinciden exactamente con `travis`

## Links
- More information: https://github.com/junegunn/fzf#usage
