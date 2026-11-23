# resolvectl

Résout les noms de domaine, les adresses IPV4 et IPv6, les enregistrements de ressources DNS et les services. Inspecte et reconfigure le résolveur DNS.

## Usage
- `resolvectl` : affiche les paramètres DNS
- `resolvectl query domaine1 domaine2 ...` : résout les adresses IPv4 et IPv6 pour un ou plusieurs domaines
- `resolvectl query adresse_ip` : récupère le domaine d'une IP spécifiée
- `resolvectl flush-caches` : vide tous les caches DNS locaux
- `resolvectl statistics` : affiche les statistiques DNS (transactions, cache et verdicts DNSSEC)
- `resolvectl --legend no -t MX query domaine` : récupère un enregistrement MX du domaine
- `resolvectl service _service._protocole nom` : résout un enregistrement SRV, par exemple `_xmpp-server._tcp gmail.com`
- `resolvectl tlsa tcp domaine:443` : récupère une clé TLS

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/resolvectl.html
