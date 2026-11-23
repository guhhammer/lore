# at

Ejecuta los comandos una vez en otro momento. Los resultados se enviarán al correo del usuario.

## Usage
- `at now + 5 minutes` : crea comandos interactivamente y los ejecuta en 5 minutos (pulsa `<Ctrl d>` cuando termines)
- `at hh:mm` : crea comandos interactivamente y los ejecuta a una hora determinada
- `echo "comando" | at 1000` : ejecuta un comando de `stdin` a las 10:00 AM de hoy
- `at -f ruta/al/archivo 9:30 PM Tue` : ejecuta comandos desde un archivo determinado el próximo martes
- `at -l` : lista todos los trabajos en cola para el usuario actual (igual que `atq`)
- `at -c número_de_trabajo` : visualiza un trabajo específico

## Links
- More information: https://manned.org/at
