# nsenter

Ejecuta un nuevo comando en el espacio de nombres de un proceso en ejecución. Particularmente útil para imágenes Docker o jaulas chroot.

## Usage
- `nsenter -t pid -a comando argumentos_del_comando` : ejecuta un comando específico utilizando los mismos espacios de nombres como un proceso existente
- `nsenter -t pid --mount|uts|ipc|net|pid|user|cgroup comando argumentos_del_comando` : ejecuta un comando específico en el espacio de nombres mount|UTS|IPC|network|PID|user|cgroup|time de un proceso existente
- `nsenter -t pid -u -T -i -- comando argumentos_del_comando` : ejecuta un comando específico en los espacios de nombres UTS, time e IPC de un proceso existente
- `nsenter -n/proc/pid/ns/net -- comando argumentos_del_comando` : ejecuta un comando específico en el espacio de nombres de un proceso existente haciendo referencia a procfs

## Links
- More information: https://manned.org/nsenter
