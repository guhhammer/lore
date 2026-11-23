# set-location

Affiche le dossier de travail actuel ou se déplace vers un autre dossier. Remarque : cette commande peut uniquement être utilisée via PowerShell.

## Usage
- `Set-Location chemin\vers\dossier` : va dans le dossier spécifié
- `Set-Location C:chemin\vers\dossier` : va dans un dossier spécifique sur un autre lecteur
- `Set-Location chemin\vers\dossier -PassThru` : va dans le dossier spécifié et affiche son emplacement
- `Set-Location ..` : remonte au dossier parent du dossier actuel
- `Set-Location ~` : va dans le dossier personnel de l’utilisateur actuel
- `Set-Location -|+` : retourne au dossier précédemment sélectionné ou avance vers le suivant
- `Set-Location \` : va à la racine du lecteur actuel
