# ssh-keygen

Generiert SSH Schlüssel für Authentifizierung, Passwort-lose Logins und mehr.

## Usage
- `ssh-keygen` : erstelle ein SSH Schlüssel-Paar interaktiv
- `ssh-keygen -t ed25519 -a 32 -f ~/.ssh/datei` : generiere ein ed25519 Schlüssel-Paar mit 32 Schlüssel-Ableitungs-Iterationen und speicher unter einem bestimmten Dateinamen
- `ssh-keygen -t rsa -b 4096 -C "kommentar|e-mail"` : generiere ein 4096 Bit langen RSA Schlüssel-Paar mit der E-Mail im Kommentarfeld
- `ssh-keygen -R externer_server` : entferne den Schlüssel eines Servers aus der `known_hosts` Datei (hilfreich wenn ein Server seinen Schlüssel aktualisiert hat und der alte somit nicht mehr gilt)
- `ssh-keygen -l -E md5 -f ~/.ssh/datei` : rufe den Fingerabdruck eines Schlüssels im MD5 Hex Format ab
- `ssh-keygen -p -f ~/.ssh/datei` : ändere das Passwort eines privaten Schlüssels
- `ssh-keygen -p -N "" -m PEM -f ~/.ssh/datei` : ändern Sie den Typ des Schlüsselformats (z. B. vom OPENSSH-Format in PEM), die Datei wird an Ort und Stelle neu geschrieben

## Links
- More information: https://man.openbsd.org/ssh-keygen
