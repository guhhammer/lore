# noti

Monitorea un proceso y activa una notificación gráfica de voz o servicio.

## Usage
- `noti tar -cjf ejemplo.tar.bz2 ejemplo/` : muestra una notificación cuando `tar` termina de comprimir archivos
- `comando_a_vigilar; noti` : muestra una notificación incluso cuando se pone después del comando a vigilar
- `noti -w identificador_del_proceso` : monitorea un proceso por PID y activa una notificación cuando el PID desaparece

## Links
- More information: https://github.com/variadico/noti/blob/main/docs/noti.md
