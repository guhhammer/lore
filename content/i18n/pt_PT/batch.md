# batch

Executar comandos num momento mais tarde quando a carga do sistema permitir. O serviço atd (ou atrun) deve correr para atuais execuções.

## Usage
- `batch` : executa comandos da entrada padrão (premir `<Ctrl d>` quando terminado)
- `echo "./criar_copia_bd.sh" | batch` : executa um comando da entrada padrão
- `batch -f caminho/para/ficheiro` : executa comandos de um dado ficheiro

## Links
- More information: https://manned.org/batch
