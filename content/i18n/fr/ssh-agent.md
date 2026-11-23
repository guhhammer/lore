# ssh-agent

Lance un agent SSH. Un agent SSH permet de stocker des clés SSH déchiffrées, jusqu'à ce qu'elle soient retirées ou que l'agent soit arrêté.

## Usage
- `eval $(ssh-agent)` : démarre un agent SSH pour le shell actuel
- `ssh-agent -k` : arrête l'agent actuellement en fonctionnement

## Links
- More information: https://man.openbsd.org/ssh-agent
