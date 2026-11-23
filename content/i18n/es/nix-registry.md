# nix registry

Administra un registro de un flake de nix.

## Usage
- `nix registry pin nixpkgs` : fija la revisión de `nixpkgs` a la versión actual del repositorio upstream
- `nix registry pin entrada github:dueño/repositorio/rama_o_revisión` : fija una entrada a la versión actual de la rama, o una revisión particular de un repositorio de GitHub
- `nix registry add entrada github:dueño/repositorio` : añade una entrada nueva que siempre apunta a la versión más reciente de un repositorio de GitHub, actualizando automáticamente
- `nix registry remove entrada` : quita una entrada del registro
- `nix registry --help` : vea la documentación sobre lo que son los registros de flakes de nix

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-registry.html
