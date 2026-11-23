# git am

Aplica archivos de parche. Útil cuando se reciben commits por correo electrónico.

## Usage
- `git am ruta/al/archivo.patch` : aplica un archivo de parche
- `curl -L https://example.com/file.patch | git am` : aplica un archivo de parche remoto
- `git am --abort` : aborta el proceso de aplicar un archivo de parche
- `git am --reject ruta/al/archivo.patch` : aplica todo lo posible de un archivo de parche y guarda los fragmentos fallidos para rechazar archivos

## Links
- More information: https://git-scm.com/docs/git-am
