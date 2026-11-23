# kill

Envía una señal a un proceso, usualmente relacionada con detener el proceso. Todas las señales a excepción de `SIGKILL` y `SIGSTOP` pueden ser interceptadas por el proceso para efectuar una salida limpia.

## Usage
- `kill identificador_del_proceso` : termina un programa usando la señal `SIGTERM` (terminar) predeterminada
- `kill -l` : lista todas las señales disponibles (para utilizarlas sin el prefijo `SIG`)
- `kill -1 identificador_del_proceso` : termina un programa usando la señal `SIGHUP` (hang up/colgar). Muchos programas residentes (daemons) se recargarán en lugar de terminar
- `kill -2 identificador_del_proceso` : termina un programa usando la señal `SIGINT` (interrumpir). Esto es normalmente iniciado por el usuario al presionar `<Ctrl c>`
- `kill -9 identificador_del_proceso` : señala al sistema operativo terminar inmediatamente un programa (el cual no tiene oportunidad de capturar la señal)
- `kill -STOP identificador_del_proceso` : señala al sistema operativo pausar un programa hasta que la señal `SIGCONT` (continuar) sea recibida
- `kill -USR1 -identificador_de_grupo` : envía una señal `SIGUSR1` a todos los procesos con un GID (id de grupo) dado

## Links
- More information: https://manned.org/kill.1posix
