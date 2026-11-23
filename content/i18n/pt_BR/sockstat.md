# sockstat

Lista sockets de domínio aberto Internet ou UNIX.

## Usage
- `sockstat -l` : vê quais usuários/processos estão escutando em quais portas
- `sockstat -4|6 -l -P tcp|udp|sctp|divert -p port1,port2...` : exibe informação para sockets IPv[4] e IPv[6] escutando em portas específicas usando um protocolo específico
- `sockstat -cnw` : exibe também sockets conectados, não resolvendo UIDs numéricos para nome de usuários e usando um campo mais largo
- `sockstat -jv` : exibe somente sockets que pertencem a um ID jail específico ou nome de modo detalhado
- `sockstat -sU` : exibe o estado do protocolo e o número da porta do encapsulamento UDP remoto, se aplicável (atualmente, estes estão implementados somente para SCTP e TCP)
- `sockstat -CS` : exibe o módulo de controle de congestionamento e a pilha de protocolo, se aplicável (atualmente, estes estão implementados somente para TCP)
- `sockstat -L` : exibe apenas sockets da Internet se os endereços local e estrangeiro não estiverem no prefixo de rede loopback 127.0.0.0/8, ou não contiverem o endereço de loopback IPv6 ::1
- `sockstat -qui` : não exibe o cabeçalho (modo silencioso), mostrando sockets unix e exibindo o `inp_gencnt`

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=sockstat
