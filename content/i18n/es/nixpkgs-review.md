# nixpkgs-review

Revisa solicitudes de incorporación de cambios en el repositorio de paquetes de NixOS (nixpkgs). Después de una construcción exitosa, un `nix-shell` con todos los paquetes se inicia.

## Usage
- `nixpkgs-review pr número_de_solicitud|url_de_solicitud` : construye paquetes cambiados en una solicitud especificada
- `nixpkgs-review pr --post-result número_de_solicitud|url_de_solicitud` : construye paquetes cambiados y publica un comentario con un reporte (requiere armar un token en `hub`, `gh`, o la variable del entorno `$GITHUB_TOKEN`)
- `nixpkgs-review pr --print-result número_de_solicitud|url_de_solicitud` : construye paquetes cambiados y muestra un reporte
- `nixpkgs-review rev HEAD` : construye paquetes cambiados en un commit local
- `nixpkgs-review wip` : construye paquetes cambiados que todavía no hayan sido commiteados
- `nixpkgs-review wip --staged` : construye paquetes cambiados que hayan sido añadidos a git

## Links
- More information: https://github.com/Mic92/nixpkgs-review#usage
