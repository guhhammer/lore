# top

Muestra información dinámica en tiempo real sobre procesos ejecutándose.

## Usage
- `top` : inicia top
- `top -i` : oculta los procesos inactivos o zombies
- `top -u usuario` : muestra solo procesos pertenecientes a un usuario dado
- `top -o nombre_columna` : ordena procesos por una columna
- `top -Hp identificador_de_proceso` : muestra los hilos individuales de un proceso dado
- `top -p $(pgrep -d ',' nombre_proceso)` : muestra solo los procesos con un(os) PID(s) dado(s), separados por comas. (Normalmente no se conoce el PID de antemano. Este ejemplo lo obtiene del nombre del proceso)
- `<?>` : obtén ayuda acerca de los comandos interactivos

## Links
- More information: https://manned.org/top
