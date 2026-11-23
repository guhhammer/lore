# disown

Permite que los subprocesos sigan activos más allá del intérprete de comandos al que están vinculados.

## Usage
- `disown` : renuncia al trabajo actual
- `disown %número_trabajo` : renuncia a un trabajo específico (ejecute `jobs` para encontrar el número de trabajo)
- `disown -a` : renuncia a todos los trabajos (solo Bash)
- `disown -h %jnúmero_trabajo` : mantiene el trabajo (no renuncia a él), pero lo marca para que no se reciba ningún `SIGHUP` futuro al salir del intérprete de comandos (solo Bash)

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-disown
