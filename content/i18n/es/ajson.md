# ajson

Ejecuta JSONPath en objetos JSON.

## Usage
- `ajson '$..json[?(@.path)]' ruta/al/archivo.json` : lee JSON de un archivo y ejecuta una expresión JSONPath especificada
- `cat ruta/al/archivo.json | ajson '$..json[?(@.path)]'` : lee JSON de `stdin` y ejecuta una expresión JSONPath especificada
- `ajson 'avg($..price)' 'https://example.com/api/'` : lee JSON de una URL y evalúa una expresión JSONPath especificada
- `echo '3' | ajson '2 * pi * $'` : lee un simple cadena JSON y calcula un valor

## Links
- More information: https://github.com/spyzhov/ajson#console-application
