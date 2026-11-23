# choom

Muestra y cambia el ajuste del OOM-killer.

## Usage
- `choom -p pid` : muestra la puntuación OOM-killer del proceso con un identificador específico
- `choom -p pid -n -1000..+1000` : modifica la puntuación OOM-killer de un proceso específico
- `choom -n -1000..+1000 comando argumento1 argumento2 ...` : ejecuta un comando con una puntuación OOM-killer específica

## Links
- More information: https://manned.org/choom
