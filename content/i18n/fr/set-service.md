# set-service

Démarre, arrête et suspend un service, et modifie ses propriétés. Remarque : cette commande peut uniquement être utilisée via PowerShell.

## Usage
- `Set-Service -Name nom_hôte -DisplayName "nom"` : modifie un nom d’affichage
- `Set-Service -Name nom_service -StartupType Automatic` : modifie le type de démarrage des services
- `Set-Service -Name nom_service -Description "description"` : modifie la description d’un service
