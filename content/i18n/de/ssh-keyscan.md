# ssh-keyscan

Rufe öffentliche SSH Schlüssel eines externen Servers ab.

## Usage
- `ssh-keyscan server` : rufe alle öffentlichen SSH Schlüssel eines Servers ab
- `ssh-keyscan -p port server` : rufe alle öffentlichen SSH Schlüssel unter einem bestimmten Port ab
- `ssh-keyscan -t rsa,dsa,ecdsa,ed25519 server` : rufe bestimmte SSH Schüssel-Typen ab
- `ssh-keyscan -H server >>~/.ssh/known_hosts` : aktualisiere die `known_hosts` SSH Datei mit dem Fingerabdruck eines bestimmten Servers

## Links
- More information: https://man.openbsd.org/ssh-keyscan
