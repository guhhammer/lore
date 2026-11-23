# autoconf

Génère des scripts de configuration pour configurer automatiquement les paquets du code source.

## Usage
- `autoconf` : génère un script de configuration depuis `configure.ac` (si présent) ou `configure.in` et le sauvegarde dans `configure`
- `autoconf fichier_de_template` : génère un script de configuration depuis un modèle spécifié; et l'affiche la sortie standard
- `autoconf -f -o fichier_de_sortie fichier_de_template` : génère un script de configuration depuis un modèle spécifié (même si the fichier d'entrée n'a pas changé) et l'écrit dans un fichier

## Links
- More information: https://manned.org/autoconf
