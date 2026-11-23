# pass

Guarda e lê senhas ou outras informações sensíveis. Todos os dados são criptografados com GPG e gerenciados por um repositório Git.

## Usage
- `pass init gpg_id_1 gpg_id_2 ...` : inicia (ou recriptografa) o armazenamento usando um ou mais IDs GPG
- `pass insert -m caminho/para/arquivo` : salva uma nova senha e informações adicionais (pressione `<Ctrl d>` em uma nova linha para completar)
- `pass edit caminho/para/dados` : edita uma entrada
- `pass -c caminho/para/dados` : copia uma senha (Primeira linha do arquivo de dados) para a área de transferência
- `pass` : lista toda a árvore de armazenamento
- `pass generate -c caminho/para/dados num` : gera uma nova senha aleatória com um comprimento (num) específico e copia para a área de transferência
- `pass git init` : inicializa um novo repositório Git (todas as mudanças feitas pelo pass são commitadas automaticamente)
- `pass git comando` : executa um comando Git no repositório de senhas

## Links
- More information: https://git.zx2c4.com/password-store/about/
