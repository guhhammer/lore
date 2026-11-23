# plantuml

Crea diagramas UML a partir de un lenguaje de texto plano y los renderiza en diferentes formatos.

## Usage
- `plantuml diagrama1.puml diagrama2.puml` : renderiza los diagramas al formato por defecto (PNG)
- `plantuml -f formato diagrama.puml` : renderiza un diagrama en un formato determinado (p.ej. `png`, `pdf`, `svg`, `txt`)
- `plantuml ruta/a/diagramas` : renderiza todos los diagramas de un directorio
- `plantuml -o ruta/a/salida diagrama.puml` : renderiza un diagrama al directorio de salida
- `plantuml -nometadata diagrama.png >diagrama.puml` : renderiza un diagrama sin almacenar el código fuente del diagrama (Nota: Se almacena por defecto cuando no se especifica la opción `-nometadata`)
- `plantuml -metadata diagrama.png >diagrama.puml` : recupera la fuente de los metadatos de un diagrama `plantuml`
- `plantuml --config config.cfg diagrama.puml` : renderiza un diagrama con el archivo de configuración
- `plantuml -h` : muestra la ayuda

## Links
- More information: https://plantuml.com/en/command-line
