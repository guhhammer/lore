# git bisect

Utiliza la búsqueda binaria para encontrar la confirmación que introdujo un error. Git salta de un lado a otro del gráfico de confirmaciones hasta alcanzar progresivamente la confirmación defectuosa.

## Usage
- `git bisect start confirmación_errónea confirmación_buena` : comienza una sesión de bisecado en un rango de confirmaciones delimitado por una confirmación errónea conocida y por una sana conocida (normalmente más antigua)
- `git bisect good|bad` : para cada confirmación que `git bisect` seleccione, marcala como "mala" (`bad`) o "buena" (`good`) después de probarla para el problema
- `git bisect reset` : termina la sesión de bisecado y vuelve a la rama anterior después de que `git-bisect` determine con precisión la confirmación defectuosa
- `git bisect skip` : omite una confirmación durante una sesión de bisecado (p. ej. una que falla las pruebas debido a un problema diferente)
- `git bisect log` : muestra un registro de lo que se ha hecho hasta el momento

## Links
- More information: https://git-scm.com/docs/git-bisect
