# csplit

Divide un archivo en partes. Esto genera archivos con los nombres `xx00`, `xx01`, y así sucesivamente.

## Usage
- `csplit ruta/al/archivo 10` : divide un archivo en dos partes, comenzando la segunda en la línea 10
- `csplit ruta/al/archivo 7 23` : divide un archivo en tres partes, comenzando las últimas partes en las líneas 7 y 23
- `csplit ruta/al/archivo 5 {*}` : inicia una nueva parte cada 5 líneas (fallará si el número de líneas no es divisible por 5)
- `csplit -k ruta/al/archivo 5 {*}` : inicia una nueva parte cada 5 líneas, ignorando el error de división exacta
- `csplit ruta/al/archivo 5 -f prefijo` : divide un archivo a partir de la línea 5 y utiliza un prefijo personalizado para los archivos de salida (el valor por defecto es `xx`)
- `csplit ruta/al/archivo /regex/` : divide un archivo a partir de la primera línea que coincida con un patrón `regex`
