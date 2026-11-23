# apt-mark

Utilitaire permettant de modifier l'état des paquets installés.

## Usage
- `sudo apt-mark auto package_name` : marque un paquet comme étant automatiquement installé
- `sudo apt-mark hold package_name` : maintient un paquet à sa version actuelle et empêche les mises à jour
- `sudo apt-mark unhold package_name` : permet une nouvelle mise à jour d'un paquet
- `apt-mark showmanual` : affiche les paquets installés manuellement
- `apt-mark showhold` : affiche les paquets détenus qui ne sont pas mis à jour

## Links
- More information: https://manned.org/apt-mark
