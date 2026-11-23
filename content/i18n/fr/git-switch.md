# git switch

Basculez entre les branches Git. Nécessite la version 2.23+ de Git.

## Usage
- `git switch nom_de_branche` : baculer sur une branche existante
- `git switch -c nom_de_branche` : créer une nouvelle branche et basculer dessus
- `git switch -c nom_de_branche commit` : créer une nouvelle branche en partant d'un commit donné et basculer dessus
- `git switch -` : basculer sur la branche précédente
- `git switch --recurse-submodules nom_de_branche` : basculer vers une branche et mettre à jour tous les sous-modules pour qu'ils correspondent
- `git switch -m nom_de_branche` : basculer vers une branche et fusionner automatiquement la branche actuelle et toutes les modifications non validées dedans

## Links
- More information: https://git-scm.com/docs/git-switch
