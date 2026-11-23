# nix develop

Ejecuta un shell de Bash que provee el entorno de construcción de una derivación.

## Usage
- `nix develop nixpkgs#pkg` : ejecuta un shell con todas las dependencias de un paquete de nixpkgs disponibles
- `nix develop` : inicia un shell para desarrollo para el paquete predeterminado de un flake en el directorio actual
- `configurePhase; buildPhase` : en ese shell, configura y construye los códigos fuentes

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-develop.html
