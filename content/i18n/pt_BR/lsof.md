# lsof

Lista arquivos abertos e os seus processos correspondentes. Nota: Privilégios de administrador (ou sudo) são necessários para listar arquivos abertos por outros.

## Usage
- `lsof caminho/para/arquivo` : localiza os processos que têm um certo arquivo aberto
- `lsof -i :porta` : localiza o processo que abriu uma porta de internet local
- `lsof -t caminho/para/arquivo` : mostra o ID (PID) do processo que abriu um arquivo especificado
- `lsof -u nome_usuario` : lista arquivos abertos por um certo usuário
- `lsof -c nome_processo_ou_comando` : lista arquivos abertos por um certo comando ou processo
- `lsof -p pid` : lista arquivos abertos por um certo processo, dado o seu PID
- `lsof +D caminho/para/diretório` : lista arquivos abertos em um diretório
- `lsof -i6TCP:porta -sTCP:LISTEN -n -P` : encontra o processo que está ouvindo uma porta de IPv6 TCP local

## Links
- More information: https://manned.org/lsof
