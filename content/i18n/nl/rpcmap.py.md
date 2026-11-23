# rpcmap.py

Zoek naar luisterende MSRPC interfaces met behulp van een string binding (bijv. `ncacn_ip_tcp:host[port]`). Onderdeel van de Impacket-suite.

## Usage
- `rpcmap.py stringbinding` : maak verbinding met een MSRPC interface met behulp van een string binding (bijv. `ncacn_ip_tcp:host[port]`)
- `rpcmap.py -brute-uuids stringbinding` : bruteforce UUID's zelfs als de MGMT interface beschikbaar is
- `rpcmap.py -brute-opnums stringbinding` : bruteforce bewerkingsnummers (opnums) voor ontdekte UUID's
- `rpcmap.py -brute-versions stringbinding` : bruteforce hoofdversies van gevonden UUID's
- `rpcmap.py -target-ip ip_adres stringbinding` : geef handmatig een doel-IP-adres op
- `rpcmap.py -auth-rpc domein/gebruikersnaam:wachtwoord stringbinding` : authenticeer naar de RPC interface met gebruikersnaam en wachtwoord
- `rpcmap.py -hashes-rpc LMHASH:NTHASH stringbinding` : authenticeer met NTLM hashes voor RPC
- `rpcmap.py -debug stringbinding` : schakel debug-uitvoer in voor uitgebreide informatie

## Links
- More information: https://github.com/fortra/impacket
