# git lfs

Trabaja con archivos grandes en repositorios de Git.

## Usage
- `git lfs install` : inicializa Git LFS
- `git lfs track '*.bin'` : rastrea archivos que coinciden con un patrón
- `git config -f .lfsconfig lfs.url url_del_punto_de_acceso_LFS` : cambia la URL a la que apunta Git LFS (útil si el servidor LFS está separado del servidor Git)
- `git lfs track` : muestra los patrones rastreados
- `git lfs ls-files` : muestra los archivos que han sido añadidos con un commit
- `git lfs push --all nombre_remoto nombre_de_la_rama` : introduce todos los objetos LFS en el servidor remoto (útil si se encuentran errores)
- `git lfs fetch` : trae todos los objetos de Git LFS
- `git lfs checkout` : verifica todos los objetos de Git LFS

## Links
- More information: https://github.com/git-lfs/git-lfs/tree/main/docs
