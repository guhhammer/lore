# systemctl

Controla el sistema systemd i el gestor de serveis.

## Usage
- `systemctl status` : mostra tots els serveis en execució
- `systemctl --failed` : llista les unitats fallides
- `systemctl start|stop|restart|reload unitat` : inicia/Atura/Reinicia/Recarrega un servei
- `systemctl status unitat` : mostra l'estat d'una unitat
- `systemctl enable|disable unitat` : habilita/Deshabilita una unitat perquè s'inicii en l'arrencada
- `systemctl mask|unmask unida` : enmascara/Desenmascara una unitat per evitar la seva habilitació i activació manual
- `systemctl daemon-reload` : recarrega systemd, buscant unitats noves o modificades
- `systemctl is-enabled unitat` : comprova si una unitat està habilitada

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
