# suspend

Suspende la ejecución del intérprete de comandos actual.

## Usage
- `bash <Enter> suspend` : suspende el intérprete de comandos actual (útil para cuando está con intérpretes de comandos anidados como `su`)
- `pkill -CONT bash` : ejecuta en un terminal separado para continuar desde la suspensión si `suspend` fue usado en un intérprete de comandos no anidado
- `suspend -f` : fuerza la suspensión, incluso si esto bloquea el sistema

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-suspend
