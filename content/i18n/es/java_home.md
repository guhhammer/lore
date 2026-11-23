# java_home

Devuelve un valor para `$JAVA_HOME` o ejecuta un comando usando esta variable.

## Usage
- `java_home --version 1.5+` : lista JVMs basadas en una versión específica
- `java_home --arch i386` : lista JVMs basadas en una [arch]itectura específica
- `java_home --datamodel Applets|WebStart|BundledApp|JNI|CommandLine` : lista JVMs basadas en tareas específicas (por defecto `CommandLine`)
- `java_home --xml` : lista JVMs en formato XML
- `java_home --help` : muestra la ayuda

## Links
- More information: https://www.unix.com/man-page/osx/1/java_home
