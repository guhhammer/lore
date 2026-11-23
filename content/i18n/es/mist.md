# mist

MIST - macOS Installer Super Tool. Descarga automáticamente Firmwares/Instaladores de macOS.

## Usage
- `mist list firmware` : lista todos los firmwares de macOS disponibles para los Mac Silicon de Apple
- `mist list installer` : lista todos los instaladores de macOS disponibles para Mac Intel, incluidos los instaladores universales para macOS Big Sur y versiones posteriores
- `mist list installer --compatible` : lista todos los instaladores de macOS compatibles con esta Mac, incluidos los instaladores universales de macOS Big Sur y posteriores
- `mist list installer --include-betas` : lista todos los instaladores de macOS disponibles para Mac Intel, incluidas las betas tanto como también los instaladores universales para macOS Big Sur y versiones posteriores
- `mist list installer --latest "macOS Sonoma"` : lista solo el último instalador de macOS Sonoma para las Macs Intel, incluidos los instaladores universales de macOS Big Sur y posteriores
- `mist list installer --export "/ruta/al/archivo_con_datos_exportados.csv"` : lista y exporta instaladores de macOS a un archivo CSV
- `mist download firmware "macOS Sonoma" --firmware-name "Install %NAME% %VERSION%-%BUILD%.ipsw"` : descarga el último firmware de macOS Sonoma para los Mac Silicon de Apple, con un nombre personalizado
- `mist download installer "13.5.2" application` : descarga una versión específica del instalador de macOS para Mac Intel, incluidos los instaladores universales de macOS Big Sur y posteriores

## Links
- More information: https://github.com/ninxsoft/mist-cli
