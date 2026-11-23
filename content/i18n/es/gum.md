# gum

Produce guiones glamorosos para el intérprete de comando.

## Usage
- `gum choose "opción_1" "opción_2" ...` : ofrece varias opciones para elegir una y la imprime en `stdout`
- `gum input --placeholder "valor"` : muestra una entrada de texto interactiva para que el usuario introduzca una cadena con un texto indicativo (placeholder) específico
- `gum confirm "¿Continuar?" --default=false --affirmative "Sí" --negative "No" && echo "Seleccionó Sí" || echo "Seleccionó No"` : abre un aviso de confirmación interactivo y sale con `<0>` o `<1>`
- `gum spin -s dot|line|minidot|jump|pulse|points|globe|moon|monkey|meter|hamburger --title "cargando..." comando` : muestra un spinner con un texto acompañante mientras se ejecuta una orden
- `gum format -t emoji ":smile: :heart: hola"` : formatea texto para incluir emojis
- `gum write >ruta/al/archivo` : solicita texto de varias líneas interactivamente y escribirlo en `datos.txt`

## Links
- More information: https://github.com/charmbracelet/gum#tutorial
