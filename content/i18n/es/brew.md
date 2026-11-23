# brew

Administrador de paquetes para macOS y Linux. Algunos subcomandos, como `install`, tienen su propia documentación de uso.

## Usage
- `brew install formula|cask` : instala la última versión estable de una fórmula
- `brew list` : lista todas las fórmulas y casks instaladas
- `brew upgrade formula|cask` : actualiza una fórmula o cask instalada (si no se indica ninguna, todas las fórmulas/casks se actualizan)
- `brew update` : trae la versión más reciente de Homebrew y todas sus fórmulas y casks desde el repositorio fuente de Homebrew
- `brew outdated` : muestra las fórmulas y casks que tienen una versión más reciente disponible
- `brew search texto` : busca fórmulas (por ej. paquetes) y casks (por ej. paquetes nativos) disponibles
- `brew info formula|cask` : muestra la información de una fórmula o un cask (versión, ruta de instalación, dependencias, etc.)
- `brew doctor` : revisa la instalación local de Homebrew en busca de problemas potenciales

## Links
- More information: https://docs.brew.sh/Manpage
