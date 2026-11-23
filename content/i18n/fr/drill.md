# drill

Effectue diverses requêtes DNS.

## Usage
- `drill example.com` : recherche des adresses IP(s) associées à un nom d'hôte (enregistrements A)
- `drill mx example.com` : recherche le(s) serveur(s) de messagerie associé(s) à un nom de domaine donné (enregistrement MX)
- `drill any example.com` : obtient tous les types d'enregistrements pour un nom de domaine donné
- `drill example.com @8.8.8.8` : spécifie un autre serveur DNS à interroger
- `drill -x 8.8.8.8` : effectue une recherche DNS inversée sur une adresse IP (enregistrement PTR)
- `drill -TD example.com` : suit la traçabilité DNSSEC des serveurs racines jusqu'à un nom de domaine
- `drill -s dnskey example.com` : affiche le(s) enregistrement(s) DNSKEY d'un nom de domaine

## Links
- More information: https://manned.org/drill
