# nix-build.3

Construye una expresión de Nix (descargando desde caché cuando sea posible).

## Usage
- `nix build nixpkgs#pkg` : construye un paquete desde nixpkgs, creando un enlace simbólico al resultado en `./result`
- `nix build -L .#pkg` : construye un paquete desde un flake en el directorio actual, mostrando el registro de construcción en el proceso
- `nix build ruta/al/directorio` : construye el paquete predeterminado de un flake en algún directorio
- `nix build --no-link --print-out-paths` : construye un paquete sin hacer el enlace simbólico `result`, mostrando a su vez la ruta del almacén de Nix en `stdout`

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-build.html
