# cpulimit

Una herramienta para limitar el uso del CPU de otros procesos.

## Usage
- `cpulimit -p 1234 -l 25%` : limita un proceso existente con PID 1234 para que solo use el 25% del CPU
- `cpulimit -e programa -l 25` : limita un programa existente por su nombre de ejecución
- `cpulimit -l 50 -- programa argumento1 argumento2 ...` : ejecuta un programa determinado y limita su uso a solo el 50% del CPU
- `cpulimit -l 50 -b -- programa` : ejecuta un programa, limita el uso del CPU a 50% y corre cpulimit en segundo plano
- `cpulimit -l 50 -k -- programa` : elimina su proceso si el uso del CPU del programa supera el 50%
- `cpulimit -l 25 -m -- programa` : regula su proceso y sus subprocesos para que ninguno supere el 25% del CPU

## Links
- More information: https://manned.org/cpulimit
