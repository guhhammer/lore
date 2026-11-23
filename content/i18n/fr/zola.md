# zola

Un générateur de site statique à partir d'un unique binaire sans dépendance.

## Usage
- `zola init mon_site` : crée la structure du répertoire utilisé par Zola dans un répertoire donné
- `zola build` : construit la totalité du site dans le répertoire `public` (si le répertoire existe, il est supprimé)
- `zola build --output-dir chemin/du/repertoire_de_sortie/` : construit la totalité du site dans un répertoire différent
- `zola serve` : construit et met à disposition le site à partir d'un serveur local (l'adresse par défaut est `127.0.0.1:1111`)
- `zola check` : construit l'ensemble des pages comme la commande `build`, sans écrire le résultat sur le disque

## Links
- More information: https://www.getzola.org/documentation/getting-started/cli-usage/
