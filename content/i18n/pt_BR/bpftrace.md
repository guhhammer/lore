# bpftrace

Linguagem de análise de alto nível para eBPF Linux.

## Usage
- `sudo bpftrace -l` : lista todos os probes
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'` : roda um programa de uma linha (e.g. número de syscalls por programa)
- `sudo bpftrace caminho/para/arquivo` : roda um programa de um arquivo
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter /pid == 123/ { @[comm] = count(); }'` : analisa um programa por PID
- `sudo bpftrace -d -e 'programa_de_uma_linha'` : mostra o resultado do programa em eBPF, sem rodar ele
- `bpftrace -V` : mostra versão

## Links
- More information: https://github.com/bpftrace/bpftrace/blob/master/man/adoc/bpftrace.adoc
