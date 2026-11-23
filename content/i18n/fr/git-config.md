# git config

Gérer les options de configuration personnalisées pour les référentiels Git. Ces configurations peuvent être locales (pour le référentiel courant) ou globales (pour l'utilisateur).

## Usage
- `git config -l --local` : liste les entrées de configurations locales (stockées dans `.git/config` du répertoire courant)
- `git config -l --global` : liste les entrées de configuration globales (stockées dans `~/.gitconfig`)
- `git config alias.unstage` : récupère la valeur d'une entrée de configuration
- `git config --global alias.unstage "reset HEAD --"` : attribue la valeur d'une entrée de configuration
- `git config --global --unset alias.unstage` : restaure la valeur d'une entrée de configuration globale à sa valeur par défaut
- `git config -e` : édite le fichier de configuration du référentiel courant dans l'éditeur par défaut
- `git config --global -e` : édite le fichier de configuration globale dans l'éditeur par défaut

## Links
- More information: https://git-scm.com/docs/git-config
