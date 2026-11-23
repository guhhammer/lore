# bc

Linguaggio per calcolatrice a precisione arbitraria.

## Usage
- `bc` : avvia una sessione interattiva
- `bc -i -l` : avvia una sessione interattiva con la libreria matematica standard abilitata
- `echo '5 / 3' | bc` : calcola un'espressione
- `bc percorso/allo/script.bc` : esegue uno script
- `echo 'scale = 10; 5 / 3' | bc` : calcola un'espressione con la scala specificata
- `echo 's|c|a|l|e(1)' | bc -l` : calcola funzioni seno/coseno/arcotangente/logaritmo_naturale/esponenziale con `mathlib`
- `echo "define factorial(n) { if (n <= 1) return 1; return n*factorial(n-1); }; factorial(10)" | bc` : esegue uno script fattoriale inline

## Links
- More information: https://manned.org/bc
