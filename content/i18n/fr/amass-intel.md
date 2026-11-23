# amass intel

Collecte des renseignements open source sur une organisation comme les noms de domaines racine et les ASNs.

## Usage
- `amass intel -addr 192.168.0.1-254` : recherche les domaines racines inclus dans une plage d'adresse IP
- `amass intel -active -addr 192.168.0.1-254` : utilise les méthodes de reconnaissance active
- `amass intel -whois -d nom_de_domaine` : recherche les noms de domaines racines reliés à un domaine
- `amass intel -org nom_de_l_organisation` : recherche les ASNs qui correspondent à une organisation
- `amass intel -asn asn` : recherche les domaines racines qui correspondent à un ASN
- `amass intel -o fichier_de_sortie -whois -d nom_de_domaine` : sauvegarde les résultats dans un fichier texte
