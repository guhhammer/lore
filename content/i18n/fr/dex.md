# dex

DesktopEntry Execution est un programme servant à générer et à exécuter des fichiers DesktopEntry de type Application.

## Usage
- `dex -a` : exécute tous les programmes dans les répertoires de démarrage automatique
- `dex -a -s chemin/vers/répertoire1:chemin/vers/répertoire2:chermin/vers/répertoire3:` : exécute tous les programmes dans les répertoires spécifiés
- `dex -a -e GNOME` : prévisualise les programmes qui seraient exécutés lors d'un démarrage automatique spécifique à GNOME
- `dex -a -d` : prévisualise les programmes qui seraient exécutés lors d'un démarrage automatique standard
- `dex -p Name chemin/vers/fichier.desktop` : prévisualise la valeur de la propriété `Name` de DesktopEntry
- `dex -c chemin/vers/fichier.destkop` : crée une DesktopEntry pour un programme dans le répertoire actuel
- `dex --term terminal chemin/vers/fichier.desktop` : exécute un programme (avec `Terminal=true` dans le fichier Desktop) dans le terminal donné

## Links
- More information: https://github.com/jceb/dex#dex
