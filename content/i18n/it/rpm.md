# rpm

RPM Package Manager.

## Usage
- `rpm -q httpd` : mostra la versione del pacchetto httpd
- `rpm -qa 'mariadb*'` : elenca le versioni di tutti i pacchetti corrispondenti
- `rpm -U path/to/package.rpm --force` : installa forzatamente un pacchetto indipendentemente dalle versioni attualmente installate
- `rpm -qf /etc/postfix/main.cf` : identifica il proprietario di un file e mostra la versione del pacchetto
- `rpm -ql kernel` : elenca i file posseduti da un pacchetto
- `rpm -qp --scripts package.rpm` : mostra gli scriptlet da un file RPM
- `rpm -Va 'php-*'` : mostra file modificati, mancanti e/o installati in modo errato dei pacchetti corrispondenti
- `rpm -q --changelog pacchetto` : mostra il changelog di un pacchetto specifico

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
