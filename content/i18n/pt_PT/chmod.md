# chmod

Alterar as permissões de acesso a um ficheiro ou diretório.

## Usage
- `chmod u+x caminho/para/ficheiro` : dá a um utilizador que possui um ficheiro o direito a executá-lo
- `chmod u+rw caminho/para/ficheiro_ou_diretório` : dá a um utilizador direitos para ler e escreve (w) num ficheiro/diretório
- `chmod g-x caminho/para/ficheiro` : remove direitos de execução de um grupo
- `chmod a+rx caminho/para/ficheiro` : dá a todos (a) os utilizadores o direito de ler e executar
- `chmod o=g caminho/para/ficheiro` : dá a outros (que não estão no grupo do dono do ficheiro) os mesmos direitos do grupo
- `chmod o= caminho/para/ficheiro` : remove todos os direitos dos outros
- `chmod -R g+w,o+w caminho/para/diretório` : muda as permissões, recursivamente, dando ao grupo e outros a possibilidade de escrever (w)
