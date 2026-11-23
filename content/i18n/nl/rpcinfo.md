# rpcinfo

Maak een RPC-oproep naar een RPC-server en rapporteer wat het vindt.

## Usage
- `rpcinfo` : toon volledige tabel van alle RPC-diensten geregistreerd op localhost
- `rpcinfo -s localhost` : toon beknopte tabel van alle RPC-diensten geregistreerd op localhost
- `rpcinfo -m` : toon tabel met statistieken van rpcbind-operaties op localhost
- `rpcinfo -l remote_nfs_server_ip mountd 2` : toon lijst van items van een bepaalde servicenaam (mountd) en versienummer (2) op een remote nfs-share
- `rpcinfo -d mountd 1` : verwijder de registratie voor versie 1 van de mountd-service voor alle transporten

## Links
- More information: https://manned.org/rpcinfo
