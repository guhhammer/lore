# git lfs

Travailler dans un registre Git avec des fichiers volumineux.

## Usage
- `git lfs install` : initialise le Git LFS
- `git lfs track '*.bin'` : suit des fichiers correspondant à un pattern
- `git config -f .lfsconfig lfs.url lfs_endpoint_url` : change l'URL du point de terminaison Git LFS (utile si le serveur LFS est séparé du serveur Git)
- `git lfs track` : liste les pattern de fichiers suivis
- `git lfs ls-files` : liste les fichiers suivis ayant été commité
- `git lfs push --all nom_distant nom_de_branche` : pousse tout les objets LFS vers le serveur distant
- `git lfs fetch` : cherche tout les objets LFS
- `git lfs checkout` : charge tout les objets LFS

## Links
- More information: https://github.com/git-lfs/git-lfs/tree/main/docs
