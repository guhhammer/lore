# set-date

Modifie l’heure système de l’ordinateur pour l’heure spécifié. Remarque : cette commande peut uniquement être utilisée via PowerShell.

## Usage
- `Set-Date -Date (Get-Date).AddDays(3)` : ajoute trois jours à la date système
- `Set-Date -Adjust -0:10:0 -DisplayHint Time` : recule l’horloge système de 10 minutes
- `$90mins = New-TimeSpan -Minutes 90; Set-Date -Adjust $90mins` : ajoute 90 minutes à l’horloge système
