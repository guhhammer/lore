# argos-translate

Una biblioteca de traducción local (offline) de código abierto y una herramienta CLI escrita en Python.

## Usage
- `argospm install translate-es_en` : instala pares de traducción del español al inglés
- `argos-translate --from-lang es --to-lang en un texto corto` : traduce un texto del español (`es`) al inglés (`en`) (Nota: sólo se admiten códigos de dos letras para los idiomas
- `cat ruta/al/archivo.txt | argos-translate --from-lang en --to-lang hi` : traduce un archivo de texto del inglés al hindi
- `argospm list` : lista todos los pares de traducción instalados
- `argospm search --from-lang en` : muestra pares de traducción del inglés que están disponibles para ser instalados
- `argospm update` : actualiza pares de paquetes de lenguaje instalados
- `argos-translate --from-lang ar --to-lang ru صورة تساوي أكثر من ألف كلمة` : traduce de `ar` a `ru` (Nota: esto requiere que se instalen los pares de traducción `translate-ar_en` y `translate-en_ru`)

## Links
- More information: https://argos-translate.readthedocs.io/en/latest/source/cli.html
