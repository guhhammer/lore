# java

Lanzador de aplicaciones Java.

## Usage
- `java nombreclase` : ejecuta un archivo Java `.class` que contiene un método principal utilizando solo el nombre de la clase
- `java -classpath ruta/a/clases1:ruta/a/clases2:. nombreclase` : ejecuta un programa Java y utiliza clases adicionales de terceros o definidas por el usuario
- `java -jar nombrearchivo.jar` : ejecuta un programa `.jar`
- `java -agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=*:5005 -jar nombrearchivo.jar` : ejecuta un programa `.jar` con depuración en espera de conexión en el puerto 5005
- `java -help` : muestra la ayuda
- `java -version` : muestra las versiones de JDK, JRE y HotSpot

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/java.html
