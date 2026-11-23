# bc

Uma linguagem de calculadora de precisão arbitrária.

## Usage
- `bc` : inicia uma sessão interativa
- `bc -i -l` : inicia uma sessão interativa com a biblioteca padrão habilitada
- `echo '5 / 3' | bc` : calcula uma expressão
- `bc caminho/para/script.bc` : executa um script
- `echo 'scale = 10; 5 / 3' | bc` : calcula uma expressão com a escala especificada
- `echo 's|c|a|l|e(1)' | bc -l` : calcula uma função seno/cosseno/arco tangente/logaritmo natural/função exponencial usando `mathlib`
- `echo "define factorial(n) { if (n <= 1) return 1; return n*factorial(n-1); }; factorial(10)" | bc` : executa um script fatorial a partir da linha de comando

## Links
- More information: https://manned.org/bc
