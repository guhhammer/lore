# vol.py

Framework forensics usado para analizar volcados de memoria volátil (RAM). Con volatility3, los complementos ahora se basan en el sistema operativo. Los ejemplos a continuación usarán Windows.

## Usage
- `python3 vol.py -f ruta/al/archivo_de_volcado_de_memoria windows.info` : obtiene información sobre un archivo de volcado de memoria
- `python3 vol.py -f ruta/al/archivo_de_volcado_de_memoria windows.pslist` : lista procesos activos
- `python3 vol.py -f ruta/al/archivo_de_volcado_de_memoria windows.hashdump` : lista hashes de usuarios en el sistema
- `python3 vol.py -f ruta/al/archivo_de_volcado_de_memoria windows.netstat` : lista conexiones de red activas
- `python3 vol.py -h` : muestra la ayuda

## Links
- More information: https://volatility3.readthedocs.io/en/latest/index.html
