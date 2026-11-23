# nix profile

Instala, actualiza y elimina paquetes de los perfiles Nix.

## Usage
- `nix profile add nixpkgs#pkg1 nixpkgs#pkg2 ...` : instala paquetes de nixpkgs en el perfil predeterminado
- `nix profile add github:owner/repo/pkg --profile ruta/al/directorio` : instala un paquete de un flake en GitHub en un perfil personalizado
- `nix profile list` : enumera los paquetes actualmente instalados en el perfil predeterminado
- `nix profile remove legacyPackages.x86_64-linux.pkg` : elimina un paquete instalado desde nixpkgs del perfil predeterminado, por nombre
- `nix profile upgrade --all` : actualiza los paquetes del perfil predeterminado a las últimas versiones disponibles
- `nix profile rollback` : revierte (cancela) la última acción en el perfil predeterminado

## Links
- More information: https://nix.dev/manual/nix/latest/command-ref/new-cli/nix3-profile.html
