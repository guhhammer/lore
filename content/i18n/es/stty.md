# stty

Establece u obtiene opciones para la interfaz de un dispositivo de terminal.

## Usage
- `stty size` : muestra el tamaño actual del terminal
- `stty -a` : muestra todos los ajustes del terminal actual
- `stty rows|cols cuenta` : establece el número de filas o columnas
- `stty -F ruta/al/archivo_del_dispositivo speed` : obtiene la velocidad de transferencia real de un dispositivo
- `stty sane` : restablece todos los modos a valores razonables para la terminal actual
- `stty raw|cooked` : cambia entre el modo sin procesar y el modo normal
- `stty -echo|echo` : activa o desactiva el eco de caracteres
- `stty --help` : muestra la ayuda

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stty-invocation.html
