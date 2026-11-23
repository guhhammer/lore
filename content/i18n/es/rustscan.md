# rustscan

Escáner de puertos rápido, escrito en Rust integrado con `nmap`. Nota: Para que algunos de los ejemplos siguientes funcionen, es necesario tener instalado `nmap`.

## Usage
- `rustscan -a ip_o_nombre_del_host` : escanea todos los puertos de una o más direcciones delimitadas por comas usando los valores predeterminados
- `rustscan --top -a dirección_o_direcciones` : escanea los top 1000 puertos con detección de servicio y versión
- `rustscan -p puerto1,puerto2,...,puertoN -a dirección_o_direcciones` : escanea una lista específica de puertos
- `rustscan -r inicio-fin -a dirección_o_direcciones` : escanea un rango específico de puertos
- `rustscan -a dirección_o_direcciones -- -O -sC` : añade argumentos de script a `nmap`
- `rustscan -b tamaño_lote -t timeout -a dirección_o_direcciones` : escanea con un tamaño de lote (por defecto: 4500) y tiempo de espera personalizado (por defecto: 1500ms)
- `rustscan --scan-order serial|random -a dirección_o_direcciones` : escanea puertos en un orden específico
- `rustscan -g -a dirección_o_direcciones` : escanea en modo "greppable" (solo imprime los puertos y no usa `nmap`)

## Links
- More information: https://github.com/bee-san/RustScan/wiki
