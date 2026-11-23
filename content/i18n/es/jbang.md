# jbang

Crea, edita y ejecuta fácilmente programas en Java autocontenidos de sólo código fuente.

## Usage
- `jbang init ruta/al/archivo.java` : inicializa una clase en Java simple
- `jbang init -t=cli ruta/al/archivo.java` : inicializa una clase en Java (útil para scripts)
- `jbang run -i` : utiliza `jshell` para explorar y utilizar un script y cualquier dependencia en un editor REPL
- `jbang edit --open=codium|code|eclipse|idea|netbeans|gitpod ruta/al/script.java` : configura un proyecto temporal para editar un script en un entorno de desarrollo integrado
- `echo 'Files.list(Paths.get("/etc")).forEach(System.out::println);' | jbang -` : ejecuta un fragmento de código en Java (Java 9 y posteriores)
- `jbang ruta/al/archivo.java comando arg1 arg2 ...` : ejecuta aplicación de línea de comandos
- `jbang app install --name nombre_del_comando ruta/al/script.java` : instala un script en un directorio en el valor de la variable de entorno `PATH` del usuario actual
- `jbang jdk install versión` : instala una versión específica del JDK para utilizarla con `jbang`

## Links
- More information: https://www.jbang.dev/documentation/jbang/latest/cli/jbang.html
