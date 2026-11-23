# nix flake

Administra los flakes de Nix.

## Usage
- `nix flake init` : crea un flake nuevo (solo el archivo `flake.nix`) usando la plantilla predeterminada, en el directorio actual
- `nix flake update` : actualiza todos las entradas (dependencias) del flake en el directorio actual
- `nix flake update entrada` : actualiza una entrada específica (dependencia) del flake en el directorio actual
- `nix flake show github:dueño/repositorio` : muestra todas las salidas de un flake en github
- `nix flake --help` : muestra la ayuda

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-flake.html
