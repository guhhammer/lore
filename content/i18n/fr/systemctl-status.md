# systemctl status

Affiche l’état des unités systemd.

## Usage
- `systemctl status unité.service|timer|socket|target|...` : affiche l’état d’une unité systemd
- `systemctl status --failed` : affiche l’état des unités ayant échoué
- `systemctl status` : liste tous les services en cours d’exécution
- `systemctl status -a` : liste toutes les unités du système
- `systemctl status -t service|timer|socket|target|...` : liste toutes les unités d’un type spécifique
- `systemctl status --state active|inactive|failed` : liste toutes les unités ayant un état spécifique
- `systemctl status unité --user` : affiche l’état d’une unité utilisateur
