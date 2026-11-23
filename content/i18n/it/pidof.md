# pidof

Ottiene l'ID di un processo a partire dal suo nome.

## Usage
- `pidof bash` : elenca gli ID di tutti i processi con un dato nome
- `pidof -s bash` : elenca un solo ID di processo con il nome specificato
- `pidof -x script.py` : elenca gli ID dei processi con un dato includendo anche gli script
- `kill $(pidof nome)` : uccide tutti i processi con un dato nome

## Links
- More information: https://manned.org/pidof
