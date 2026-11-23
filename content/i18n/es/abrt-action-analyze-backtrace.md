# abrt-action-analyze-backtrace

Analiza un backtrace de C/C++. Genera un hash de duplicación, una clasificación del backtrace e identifica la función que causó el fallo. Guarda los datos como nuevos elementos `duphash`, `rating`, `crash_function` en el directorio del problema.

## Usage
- `abrt-action-analyze-backtrace` : analiza el backtrace para el directorio de trabajo actual
- `abrt-action-analyze-backtrace -d ruta/al/directorio` : analiza el backtrace para un directorio específico
- `abrt-action-analyze-backtrace -v` : analiza el backtrace de manera detallada

## Links
- More information: https://manned.org/abrt-action-analyze-backtrace
