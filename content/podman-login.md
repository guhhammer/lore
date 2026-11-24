# podman login

Log in to a container registry. Note: The default authfile path on Linux is `$XDG_RUNTIME_DIR/containers/auth.json`, which is usually stored in a `tmpfs` (in RAM).

## Usage
- `podman login registry.example.org` : log in to a registry (non-persistent on Linux; persistent on Windows/macOS)
- `podman login --authfile $HOME/.config/containers/auth.json registry.example.org` : log in to a registry persistently on Linux
- `podman login --tls-verify false registry.example.org` : log in to an insecure (HTTP) registry

## Categories
containers, users, monitoring

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-login.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-login
- Wikipedia: https://en.wikipedia.org/wiki/Podman
