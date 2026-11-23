# logger

Añade mensajes al registro del sistema.

## Usage
- `logger mensaje` : registra un mensaje en syslog
- `echo entrada_de_registro | logger` : toma la entrada de `stdin` y la registra en syslog
- `echo entrada_de_registro | logger -n hostname -P puerto` : envía la salida a un servidor syslog remoto que se ejecuta en un puerto determinado. El puerto por defecto es 514
- `echo entrada_de_registro | logger -t etiqueta` : utiliza una etiqueta específica para cada línea registrada. Por defecto es el nombre del usuario conectado
- `echo entrada_de_registro | logger -p user.warning` : registra mensajes con una prioridad determinada. Por defecto es `user.notice`. Vea `man logger` para todas las opciones de prioridad

## Links
- More information: https://manned.org/logger
