# py

Lanzador de Python para Windows que se ejecuta con la versión de Python especificada.

## Usage
- `py argumentos_python` : inicia un REPL (shell interactivo), opcionalmente con argumentos soportados por `python` (como `-c`, `-m`, etc.)
- `py ruta/al/archivo.py` : ejecuta un archivo Python específico
- `py -2|-3.7|...` : ejecuta una versión específica de Python. Si falta la versión y la variable de entorno `PYLAUNCHER_ALLOW_INSTALL` está configurada, instalar automáticamente a través de Microsoft Store o Winget
- `py --list` : lista las versiones de Python instaladas

## Links
- More information: https://docs.python.org/using/windows.html#python-launcher-for-windows
