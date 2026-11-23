# chmod

Muda a permissão de acesso de um arquivo ou diretório.

## Usage
- `chmod u+x caminho/para/arquivo` : dá ao usuário dono de um arquivo o direito de executá-lo
- `chmod u+rw arquivo_ou_diretorio` : dá ao usuário direitos para ler e wescrever em um arquivo/diretório
- `chmod g-x arquivo` : remove direitos executáveis de um grupo
- `chmod a+rx caminho/para/arquivo` : dá a atodos os usuários direitos para ler e executar
- `chmod o=g caminho/para/arquivo` : dá para outros (que não estejam no grupo do proprietário do arquivo) os mesmos direitos que o grupo
- `chmod o= caminho/para/arquivo` : remove todos os direitos de outros
- `chmod -R g+w,o+w diretorio` : muda recursivamente as permissões, dando para grupo e outros a habilidade para wescrever
- `chmod -R a+rX diretorio` : recursivamente concede a atodos os usuários permissões de leitura para arquivos e eXecute permissões para sub-diretórios dentro de um diretório
