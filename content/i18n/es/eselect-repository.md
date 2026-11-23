# eselect repository

Un módulo `eselect` para configurar repositorios ebuild para Portage. Después de habilitar un repositorio, tienes que ejecutar `emerge --sync repo_name` para descargar ebuilds.

## Usage
- `eselect repository list` : lista todos los repositorios ebuild registrados en <https://repos.gentoo.org>
- `eselect repository list -i` : lista de repositorios habilitados
- `eselect repository enable name|index` : habilita un repositorio de la lista según su nombre o índice desde el comando `list`
- `eselect repository add nombre rsync|git|mercurial|svn|... sync_uri` : activa un repositorio no registrado
- `eselect repository disable repo1 repo2 ...` : deshabilita repositorios sin eliminar su contenido
- `eselect repository remove repo1 repo2 ...` : desactiva repositorios y elimina su contenido
- `eselect repository create nombre ruta/al/repo` : crea un repositorio local y lo habilita

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect/Repository
