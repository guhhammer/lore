# git revert

Crea nuevas confirmaciones que revierten el efecto de los anteriores.

## Usage
- `git revert HEAD` : revierte la confirmación más reciente
- `git revert HEAD~4` : revierte la quinta confirmación más reciente
- `git revert 0c01a9` : revierte una confirmación específica
- `git revert nombre_rama~5..nombre_rama~2` : revierte múltiples confirmaciones
- `git revert -n 0c01a9..9a1743` : revierte confirmaciones sin crear nuevas confirmaciones

## Links
- More information: https://git-scm.com/docs/git-revert
