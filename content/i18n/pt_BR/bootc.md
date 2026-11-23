# bootc

Inicializa e atualiza seu sistema usando imagens de containeres. Manipula atualizações transacionais e transparentes utilizando imagens de containeres OCI/Docker.

## Usage
- `bootc status` : mostra todos os deployments na ordem que eles aparecem na inicialização
- `bootc upgrade --check` : mostra se há alguma atualização disponível
- `bootc upgrade --apply` : atualiza e reinicia o sistema
- `bootc switch imagem` : move seu sistema para outra base
- `bootc rollback` : reinicia o seu sistema no deployment anterior

## Links
- More information: https://manned.org/bootc
