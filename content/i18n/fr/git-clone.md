# git clone

Clone un dépôt existant.

## Usage
- `git clone emplacement_dépôt_distant chemin/vers/répertoire` : clone un dépôt existant dans un répertoire spécifique (le répertoire par défaut porte le nom du dépôt)
- `git clone --recursive emplacement_dépôt_distant` : clone un dépôt existant et ses sous-modules
- `git clone -n emplacement_dépôt_distant` : clone uniquement le repertoire `.git` d'un depot existant
- `git clone -l chemin/vers/dépôt_local` : clone un dépôt local
- `git clone -q emplacement_dépôt_distant` : clone silencieusement
- `git clone --depth 10 emplacement_dépôt_distant` : clone un dépôt existant en ne récupérant que les 10 validations les plus récentes sur la branche par défaut (plus rapide)
- `git clone -b nom_branche --single-branch emplacement_dépôt_distant` : clone un dépôt existant en ne récupérant qu'une branche spécifique
- `git clone -c core.sshCommand="ssh -i chemin/vers/clé_ssh_privée" emplacement_dépôt_distant` : clone un dépôt existant en utilisant une commande SSH spécifique

## Links
- More information: https://git-scm.com/docs/git-clone
