# ps

Información sobre procesos en ejecución.

## Usage
- `ps aux` : lista todos los procesos en ejecución
- `ps auxww` : lista todos los procesos en ejecución incluyendo el comando completo
- `ps aux | grep cadena` : busca un proceso que coincida con la cadena de texto
- `ps -u $(id -u) -F` : lista todos los procesos del usuario actual en formato supercompleto
- `ps -u $(id -u) f` : lista todos los procesos del usuario actual como un árbol
- `ps -o ppid= -p pid` : obtén el PID del proceso padre
- `ps --sort size` : ordena los procesos por consumo de memoria

## Links
- More information: https://manned.org/ps
