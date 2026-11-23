# copr-cli

Interfaz con la instancia copr de Fedora-Projects para construir RPMs y publicarlos.

## Usage
- `copr-cli whoami` : muestra al usuario conectado a copr
- `copr-cli build repositorio ruta/al/archivo_de_especificaciones` : genera un archivo spec local en copr
- `copr-cli list-builds repositorio` : comprueba el estado de las compilaciones
- `copr-cli buildscm repositorio --clone-url https://git.example.org/repo --spec nombre_archivo_espec` : activa una compilación copr de un archivo de especificaciones desde un repositorio público (Git)

## Links
- More information: https://manned.org/copr-cli
