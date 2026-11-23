# cpulimit

Eina per limitar l'ús de la CPU en altres processos.

## Usage
- `cpulimit -p 1234 -l 25%` : limita un procés existent amb PID 1234 perquè només utilitzi el 25% de CPU
- `cpulimit -e programa -l 25` : limita un programa existent per el seu nom d'execució
- `cpulimit -l 50 -- programa argument1 argument2 ...` : executa un programa determinat i limita el seu ús a només el 50% de la CPU
- `cpulimit -l 50 -b -- programa` : executa un programa, limita l'ús de la CPU a 50% i executa cpulimit en segon pla
- `cpulimit -l 50 -k -- programa` : mata el procés si l'ús de CPU del programa supera el 50%
- `cpulimit -l 25 -m -- programa` : regula el seu procés i els subprocessos perquè cap superi el 25% de CPU

## Links
- More information: https://manned.org/cpulimit
