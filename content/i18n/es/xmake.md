# xmake

Una utilidad de compilación multiplataforma C & C++ basada en Lua.

## Usage
- `xmake create -l c -P nombre_del_proyecto` : crea un proyecto Xmake C, consistente en un hello world y `xmake.lua`
- `xmake b r` : construye y ejecuta un proyecto Xmake
- `xmake r nombre_del_objetivo` : ejecuta directamente un objetivo Xmake compilado
- `xmake f -p macosx|linux|iphoneos|... -a x86_64|i386|arm64|... -m debug|release` : configura los objetivos de compilación de un proyecto
- `xmake i -o ruta/al/directorio` : instala el objetivo compilado en un directorio

## Links
- More information: https://github.com/xmake-io/xmake#command-line-interface-reference
