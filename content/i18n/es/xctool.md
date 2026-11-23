# xctool

Herramienta para construir proyectos Xcode.

## Usage
- `xctool -project TuProyecto.xcodeproj -scheme SuEsquema build` : construye un proyecto sin ningún espacio de trabajo
- `xctool -workspace TuEspacioDeTrabajo.xcworkspace -scheme TuEsquema build` : construye un proyecto que forma parte de un espacio de trabajo
- `xctool -workspace TuEspacioTrabajo.xcworkspace -scheme TuEsquema clean build test` : limpia, construye y ejecuta todas las pruebas

## Links
- More information: https://github.com/facebookarchive/xctool#usage
