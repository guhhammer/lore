# sudo

Esegue un singolo comando come superuser o come un altro utente.

## Usage
- `sudo less /var/log/syslog` : esegui un comando come superuser
- `sudo -e /etc/fstab` : modifica un file come superuser con il tuo editor di default
- `sudo -u utente -g gruppo id -a` : esegui un comando come un altro utente e/o gruppo
- `sudo !!` : ripeti l'ultimo comando prefissandolo con "sudo" (funziona solo in Bash, Zsh, ecc)
- `sudo -i` : fai partire la shell di default con i privilegi da superuser

## Links
- More information: https://www.sudo.ws/sudo.html
