# bc

Un lenguaje de calculadora de precisión arbitraria.

## Usage
- `bc` : inicia una sesión interactiva
- `bc -i -l` : inicia una sesión interactiva con la biblioteca matemática estándar activada
- `echo '5 / 3' | bc` : calcula una expresión
- `bc ruta/al/script.bc` : ejecuta un script
- `echo 'scale = 10; 5 / 3' | bc` : calcula una expresión con la escala especificada
- `echo 's|c|a|l|e(1)' | bc -l` : calcula una función seno/coseno/arctangente/logaritmo natural/exponencial utilizando `mathlib`
- `echo "define factorial(n) { if (n <= 1) return 1; return n*factorial(n-1); }; factorial(10)" | bc` : ejecuta un guión factorial en línea (inline)

## Links
- More information: https://manned.org/bc
