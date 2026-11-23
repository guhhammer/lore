# kde-builder

Construye fácilmente componentes de KDE desde tus repositorios fuente. Sustituye a `kdesrc-build`.

## Usage
- `kde-builder --initial-setup` : inicializa `kde-builder`
- `kde-builder nombre_componente1 nombre_componente2 ...` : compila un componente KDE y sus dependencias desde el código fuente
- `kde-builder -SD nombre_componente` : compila un componente sin actualizar el código local y sin compilar sus dependencias
- `kde-builder -r nombre_componente` : actualiza los directorios de compilación antes de compilar
- `kde-builder -f dependencia_componente nombre_componente` : reanuda la compilación a partir de una dependencia determinada
- `kde-builder --run nombre_ejecutable` : ejecuta un componente con un nombre de ejecutable determinado
- `kde-builder --install-login-session-only` : construye todos los componentes configurados
- `kde-builder --no-stop-on-failure nombre_ejecutable` : utiliza las bibliotecas del sistema en lugar de un componente si no se puede compilar

## Links
- More information: https://kde-builder.kde.org/en/cmdline/supported-cmdline-params.html
