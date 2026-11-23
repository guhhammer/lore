# devenv

Entornos de desarrollo rápidos, declarativos, reproducibles y componibles utilizando Nix.

## Usage
- `devenv init` : inicializa el entorno
- `devenv shell --impure` : entra en el entorno de desarrollo con hermeticidad relajada (estado de ser hermético)
- `devenv info --verbose` : obtén información detallada sobre el entorno actual
- `devenv up --config ruta/al/archivo` : inicia procesos con `devenv`
- `devenv --clean --offline` : limpia las variables de entorno y vuelve a entrar en el intérprete de comandos en el modo sin conexión
- `devenv gc` : borra las generaciones anteriores del intérprete de comandos

## Links
- More information: https://devenv.sh/getting-started/#commands
