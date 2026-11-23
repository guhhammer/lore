# systemctl

Contrôle le système systemd et le gestionnaire de services. Certaines sous-commandes, telles que disable, status, reboot, etc., disposent de leur propre documentation d’utilisation.

## Usage
- `systemctl status` : affiche tous les services actuellement actifs
- `systemctl --failed` : liste des unités en échec
- `systemctl start|stop|restart|reload unité` : démarre/Arrête/Redémarre/Recharge un service
- `systemctl enable|disable unité` : active/désactive une unité à démarrer au démarrage
- `systemctl daemon-reload` : rechargement de systemd, recherche d'unités nouvelles ou modifiées
- `systemctl is-active|is-enabled|is-failed unité` : vérifie si une unité est en cours d'exécution/activée/en échec
- `systemctl list-units -t service|socket|automount|... --state failed|running` : liste toutes les unités de service, de socket, et d’automontage en filtrant selon leur état (en échec/active)
- `systemctl cat|edit unité` : affiche/modifie le contenu d’un fichier d’unité

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
