# mosh

Mobile Shell (`mosh`) es un reemplazo robusto y receptivo para SSH. `mosh` persiste las conexiones con servidores remotos mientras deambula en las redes.

## Usage
- `mosh usuario@equipo_remoto` : conecta a un servidor remoto
- `mosh --ssh="ssh -i ruta/al/archivo_de_clave" usuario@equipo_remoto` : conecta a un servidor remoto con una identidad específica (clave privada)
- `mosh --ssh="ssh -p 2222" usuario@equipo_remoto` : conecta a un servidor remoto usando un puerto específico
- `mosh equipo_remoto -- comando -con --opciones` : ejecuta un comando en un servidor remoto
- `mosh -p 124 usuario@equipo_remoto` : selecciona un puerto UDP Mosh (útil cuando el `equipo_remoto` está tras un NAT)
- `mosh --server=ruta/a/bin/mosh-server equipo_remoto` : se lo usa cuando el binario `mosh-server` no se encuentra en la ruta estándar

## Links
- More information: https://manned.org/mosh
