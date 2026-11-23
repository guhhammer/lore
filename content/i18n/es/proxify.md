# proxify

Un proxy versátil y portátil para capturar, manipular y reproducir tráfico HTTP/HTTPS sobre la marcha.

## Usage
- `proxify` : inicia un proxy HTTP (en la interfaz de red loopback `127.0.0.1` y puerto `8888`)
- `proxify -ha "dirección_ip:número_de_puerto"` : inicia un proxy HTTP en una interfaz de red y puerto personalizados (puede requerir `sudo` para un número de puerto inferior a `1024`)
- `proxify -of jsonl|yaml -o ruta/al/archivo` : especifica el formato y el archivo de salida
- `proxify -h` : muestra la ayuda

## Links
- More information: https://github.com/projectdiscovery/proxify#usage
