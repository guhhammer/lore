# devfsadm

Administre `/dev`. Maintient l'espace de noms `/dev`.

## Usage
- `devfsadm -c disk` : recherche de nouveaux disques
- `devfsadm -C -v` : nettoie toute pendaison `/dev` liens et recherche un nouvel appareil
- `devfsadm -C -v -n` : marche à sec - sort ce qui serait changé mais ne fait aucune modification

## Links
- More information: https://www.unix.com/man-page/sunos/1m/devfsadm
