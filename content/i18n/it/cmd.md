# cmd

L'interprete dei comandi di Windows.

## Usage
- `cmd` : lancia una nuova istanza dell'interprete dei comandi
- `cmd /c echo cmd è eseguito` : esegue il comando specificato e poi esce
- `cmd /k echo cmd è eseguito` : esegue il comando specificato e poi apre una shell interattiva
- `cmd /q` : disabilita l'uso di `echo` nell'output di un comando
- `cmd /e:on|off` : abilita o disabilita le estensioni ai comandi
- `cmd /v:on|off` : abilita o disabilita l'espansione delle variabili d'ambiente
- `cmd /u` : forza l'encoding delle stringhe in Unicode per l'output
