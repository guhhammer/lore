# xcodes runtimes

Gestiona los tiempos de ejecución de Xcode Simulator.

## Usage
- `xcodes runtimes --include-betas` : muestra todos los tiempos de ejecución del simulador disponibles
- `xcodes runtimes download nombre_del_tiempo_de_ejecución` : descarga un tiempo de ejecución para el Simulator
- `xcodes runtimes install nombre_del_tiempo_de_ejecución` : descarga e instala un tiempo de ejecución para el Simulator
- `xcodes runtimes download|install "iOS|watchOS|tvOS|visionOS versión_de_tiempo_de_ejecución"` : descarga/instala un tiempo de ejecución del Simulator para una versión específica de iOS/watchOS/tvOS/visionOS (debe escribirse distinguiendo entre mayúsculas y minúsculas)
- `xcodes runtimes download|install nombre_del_tiempo_de_ejecución --directory ruta/al/directorio` : establece una ubicación específica en la que se descargará primero el archivo de tiempo de ejecución (por defecto es `~/Downloads`)
- `xcodes runtimes install nombre_del_tiempo_de_ejecución --keep-archive` : no elimina el archivo descargado cuando el Simulator se instala correctamente

## Links
- More information: https://github.com/xcodesorg/xcodes#commands
