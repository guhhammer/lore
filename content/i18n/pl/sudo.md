# sudo

Wykonuje pojedyncze polecenie jako superuser lub inny użytkownik.

## Usage
- `sudo less /var/log/syslog` : uruchom polecenie jako superuser
- `sudo -e /etc/fstab` : edytuj plik jako superuser w domyślnym edytorze
- `sudo -u uzytkownik -g grupa id -a` : uruchom polecenie jako inny użytkownik i/lub grupa
- `sudo !!` : powtórz ostatnie polecenie poprzedzone `sudo` (tylko w Bash, Zsh, etc.)
- `sudo -i` : uruchom domyślną powłokę z uprawnieniami superuser

## Links
- More information: https://www.sudo.ws/sudo.html
