# set-clipboard

Commande PowerShell permettant de placer du contenu dans le presse-papiers.

## Usage
- `Set-Clipboard -Value "texte"` : copie du texte dans le presse-papiers
- `Set-Clipboard -Value @("texte 1", "texte 2", "texte 3")` : copie plusieurs textes dans le presse-papiers, séparés par une nouvelle ligne
- `Set-Clipboard -Path "chemin\vers\fichiers_ou_dossiers"` : copie des fichiers ou des dossiers dans le presse-papiers
- `Set-Clipboard -Path "chemin\vers\fichier1","chemin\vers\fichier2","chemin\vers\fichier3"` : copie plusieurs fichiers
- `Set-Clipboard ""` : efface le presse-papiers
