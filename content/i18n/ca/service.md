# service

Gestiona els serveis mitjançant l'execució de scripts init. S'ha d'ometre la ruta completa del script (s'assumeix `/etc/init.d`).

## Usage
- `service --status-all` : llista el nom i l'estat de tots els serveis
- `service nom_del_servei start|stop|restart|reload` : inicia/Para/Reinicia/Recarrega servei (start/stop hauria d'estar sempre disponible)
- `service nom_del_servei --full-restart` : fa un reinici complet (executa el script dues vegades amb start i stop)
- `service nom_del_servei status` : mostra l'estat actual d'un servei

## Links
- More information: https://manned.org/service
