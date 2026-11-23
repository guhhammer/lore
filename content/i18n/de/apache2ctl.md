# apache2ctl

Das CLI Tool um den Apache HTTP Web Server zu administrieren. Dieser Befehl wird mit Debian-basierten Betriebssystemen geliefert, für RHEL siehe `httpd`.

## Usage
- `sudo apache2ctl start` : starte den Apache daemon. Gibt einen Fehler aus, wenn er bereits läuft
- `sudo apache2ctl stop` : stoppe den Apache Daemon
- `sudo apache2ctl restart` : starte den Apache Daemon neu
- `sudo apache2ctl -t` : überprüfe die Syntax einer Konfigurationsdatei
- `sudo apache2ctl -M` : liste alle geladenen Module auf

## Links
- More information: https://manned.org/apache2ctl
