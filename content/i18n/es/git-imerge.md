# git imerge

Ejecuta una fusión o rebase entre dos ramas Git incrementalmente. Los conflictos entre las ramas se rastrean a pares de commits individuales para simplificar la resolución de conflictos.

## Usage
- `git imerge rebase rama_a_rebasar` : inicia un rebase de tipo imerge (primero comprueba la rama a ser rebasada)
- `git imerge merge rama_a_fusionar` : inicia una fusión de tipo imerge (primero comprueba la rama en la que fusionar)
- `git imerge diagram` : muestra una diagrama ASCII para la fusión o rebase en proceso
- `git imerge continue --no-edit` : continua la operación imerge después de resolver los conflictos (primero añade con `git add` los archivos conflictivos)
- `git imerge finish` : concluye una operación imerge después de que todos los conflictos se hayan resuelto
- `git imerge remove && git checkout rama_anterior` : aborta una operación imerge y vuelve a la rama anterior

## Links
- More information: https://github.com/mhagger/git-imerge
