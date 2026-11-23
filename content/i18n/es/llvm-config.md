# llvm-config

Obtiene variada información de configuración necesaria para compilar programas que utilizan LLVM. Típicamente llamado desde sistemas de construcción, como Makefiles o scripts de configuración.

## Usage
- `clang++ $(llvm-config --cxxflags --ldflags --libs) --output ruta/al/resultado_ejecutable ruta/a/source.cc` : compila y vincula un programa basado en LLVM
- `llvm-config --prefix` : imprime el `PREFIJO` de su instalación LLVM
- `llvm-config --targets-built` : imprime todos los objetivos soportados por su LLVM instalado

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-config.html
