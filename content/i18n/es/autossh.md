# autossh

Ejecuta, monitorea y reinicia conexiones SSH. Auto-reconecta para mantener los túneles de reenvío de puertos. Acepta todas las señales SSH.

## Usage
- `autossh -M puerto_monitor "comando_ssh"` : inicia una sesión SSH, reiniciando cuando un puerto de monitoreo no retorna datos
- `autossh -M puerto_monitor -L puerto_local:localhost:puerto_remoto usuario@host` : reenvía un puerto local a uno remoto, reiniciando cuando sea necesario
- `autossh -f -M puerto_monitor -N "comando_ssh"` : crea un proceso `autossh` en segundo plano antes de ejecutar SSH y no abre un shell remoto
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" "comando_ssh"` : ejecuta en segundo plano, sin puerto de monitorización, y en su lugar envía paquetes SSH keep-alive cada 10 segundos para detectar fallos
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" -o ExitOnForwardFailure=yes -L local_port:localhost:puerto_remoto usuario@host` : ejecuta en segundo plano, sin puerto de monitorización y sin shell remoto, saliendo si falla el reenvío de puerto
- `AUTOSSH_DEBUG=1 AUTOSSH_LOGFILE=ruta/al/autossh_log_file.log autossh -f -M puerto_monitor -v -E ruta/al/archivo_ssh_log.log comando_ssh` : se ejecuta en segundo plano, registrando la salida de depuración `autossh` y la salida detallada SSH en archivos

## Links
- More information: https://manned.org/autossh
