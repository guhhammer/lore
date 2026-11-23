# podman login

Inicia sesión en un registro de contenedores (container registry). Nota: la ruta predeterminada de archivo de autenticación (authfile) en Linux es `$XDG_RUNTIME_DIR/containers/auth.json`, que generalmente se almacena en un `tmpfs` (en RAM).

## Usage
- `podman login registry.example.org` : inicia sesión en un registro (no permanente en Linux; persistente en Windows/macOS)
- `podman login --authfile $HOME/.config/containers/auth.json registry.example.org` : inicia sesión en un registro persistentemente en Linux
- `podman login --tls-verify false registry.example.org` : inicia sesión en un registro inseguro (HTTP)

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-login.1.html
