# vcpkg

Gestor de paquetes para librerías C/C++. Nota: los paquetes no se instalan en el sistema. Para usarlos, necesitas decirle a tu sistema de compilación (por ejemplo CMake) que use `vcpkg`.

## Usage
- `vcpkg install curl` : construye y añade el paquete `libcurl` al entorno de `vcpkg`
- `vcpkg install --triplet=wasm32-emscripten zlib` : construye y añade `zlib` usando la cadena de herramientas `emscripten`
- `vcpkg search nombre_del_paquete` : busca un paquete
- `cmake -B build -DCMAKE_TOOLCHAIN_FILE=ruta/al/directorio_de_instalación_vcpkg/scripts/buildsystems/vcpkg.cmake` : configura un proyecto CMake para utilizar los paquetes de `vcpkg`

## Links
- More information: https://learn.microsoft.com/vcpkg/
