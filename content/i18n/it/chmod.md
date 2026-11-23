# chmod

Cambia i permessi di accesso di file o directory.

## Usage
- `chmod u+x percorso/del/file` : dai al proprietario di un file il permesso di execute
- `chmod u+rw percorso/del/file_o_directory` : dai al proprietario i permessi di read e write su un file/directory
- `chmod g-x percorso/del/file` : rimuovi i permessi di execute dal gruppo
- `chmod a+rx percorso/del/file` : dai a tutti gli utenti i permessi di read e execute
- `chmod o=g percorso/del/file` : dai ad altri utenti (non nel gruppo proprietario del file) gli stessi diritti del gruppo
- `chmod o= percorso/del/file` : rimuovi tutti i permessi per gli altri
- `chmod -R g+w,o+w percorso/della/directory` : cambia i permessi ricorsivamente dando al gruppo e agli ottri la possibilità di write
- `chmod -R a+rX percorso/della/directory` : ricorsivamente dà a tutti gli utenti i permessi di read sui file. Dà anche i permessi di eXecute ai file che hanno almeno un permesso di esecuzione e a tutte le sottodirectory
