# act

Exécute des GitHub Actions en local avec Docker.

## Usage
- `act -l` : liste les jobs disponibles
- `act` : exécute l'événement par défault
- `act type_d_événement` : exécute un événement spécifique
- `act -j id_job` : exécute un job spécifique
- `act -n` : ne lance pas les actions maintenant (e.g un essai)
- `act -v` : affiche le journal en mode verbeux
- `act push -W path/to/workflow` : exécute un Workflow en particulier, avec l'événement push

## Links
- More information: https://manned.org/act
