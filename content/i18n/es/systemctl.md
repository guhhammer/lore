# systemctl

Controla el gestor de sistemas y servicios systemd.

## Usage
- `systemctl status` : muestra todos los servicios en ejecución
- `systemctl --failed` : lista las unidades fallidas
- `systemctl start|stop|restart|reload|status unidad` : inicia, para, reinicia, recarga o muestra el estado de una unidad
- `systemctl enable|disable unidad` : habilita o deshabilita una unidad para iniciarla al arrancar
- `systemctl daemon-reload` : reinicia systemd, lee unidades nuevas o modificadas
- `systemctl is-active|is-enabled|is-failed unidad` : checa si una unidad está activa, habilitada, o en estado fallido
- `systemctl list-units -t service|socket|automount --state failed|running` : lista todos los servicios, sockets, unidades auto-montadas filtradas por estado en ejecución o fallido
- `systemctl cat unidad` : muestra los contenidos y la ruta absoluta del archivo de una unidad

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
