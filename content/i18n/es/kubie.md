# kubie

Permite saltar entre contextos y espacios de nombres de `kubectl`.

## Usage
- `kubie ctx` : muestra un menú seleccionable de contextos
- `kubie ctx contexto` : cambia el intérprete de comandos actual al contexto dado
- `kubie ns espacio_de_nombres` : cambia el intérprete de comandos actual al espacio de nombres dado
- `kubie ctx contexto -n espacio_de_nombres` : cambia el intérprete de comandos actual al contexto y espacio de nombres dados
- `kubie exec contexto espacio_de_nombres comando` : ejecuta un comando en el contexto y espacio de nombres dados, sin crear un nuevo intérprete de comandos
- `kubie lint` : busca errores en los archivos de configuración de Kubernetes

## Links
- More information: https://github.com/sbstp/kubie#usage
