# chcon

Cambia contesto di sicurezza SELinux di file o directory.

## Usage
- `ls -lZ percorso/del/file` : mostra il contesto di sicurezza di un file
- `chcon --reference file_di_riferimento file` : cambia il contesto di sicurezza di un file usandone un'altro come riferimento
- `chcon utente:ruolo:tipo:range/livello file` : cambia l'intero contesto di sicurezza SELinux di un file
- `chcon -u utente file` : cambia solo l'utente di un contesto di sicurezza SELinux
- `chcon -r ruolo file` : cambia solo il ruolo di un contesto di sicurezza SELinux
- `chcon -t tipo file` : cambia solo il tipo di un contesto di sicurezza SELinux
- `chcon -l range/livello file` : cambia solo il range/livello di un contesto di sicurezza SELinux
