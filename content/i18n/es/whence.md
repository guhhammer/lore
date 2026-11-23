# whence

Un comando integrado de zsh para indicar cómo se interpretaría un comando dado.

## Usage
- `whence "comando"` : interpreta `comando`, con expansión si se define como un `alias` (similar al `command -v` integrado)
- `whence -v "comando"` : muestra tipo de `comando`, con localización si se define como una función, o binario (equivalente a los `type` y `command -V` integrados)
- `whence -c "comando"` : igual que el anterior, excepto que muestra el contenido de las funciones del shell en lugar de la ubicación (equivalente al `which` integrado)
- `whence -ca "comando"` : igual que el anterior, pero muestra todas las apariciones en la ruta del comando (equivalente al `where` integrado)
- `whence -p "comando"` : busca un comando en la variable de entorno `$PATH`, ignorando los comandos integrados, aliases o funciones del shell (equivalente al comando `where`)
