# make

Ejecutor de tareas para objetivos descritos en Makefile. Se utiliza principalmente para controlar la compilación de un ejecutable a partir del código fuente.

## Usage
- `make` : llama al primer objetivo especificado en el Makefile (normalmente llamado "all")
- `make objetivo` : llama a un objetivo específico
- `make -j 4 objetivo` : llama a un objetivo específico, ejecutando 4 trabajos a la vez en paralelo
- `make -f ruta/al/archivo` : utiliza un Makefile específico
- `make -C ruta/al/directorio` : ejecuta make desde otro directorio
- `make -B objetivo` : fuerza la creación de un objetivo, incluso si los archivos de origen no se han modificado
- `make objetivo variable=nuevo_valor` : anula una variable definida en el Makefile
- `make -e objetivo` : anula variables definidas en el Makefile por el entorno

## Links
- More information: https://www.gnu.org/software/make/manual/make.html
