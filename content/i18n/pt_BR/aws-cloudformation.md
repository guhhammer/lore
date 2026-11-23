# aws cloudformation

Modela, provisiona e gerencia recursos AWS, e de terceiros, ao tratar a infraestrutura como código.

## Usage
- `aws cloudformation create-stack --stack-name nome_da_pilha --region região --template-body file://caminho/para/arquivo.yml --profile perfil` : cria uma pilha a partir de um arquivo de modelo
- `aws cloudformation delete-stack --stack-name nome_da_pilha --profile perfil` : deleta uma pilha
- `aws cloudformation list-stacks --profile perfil` : lista todas as pilhas
- `aws cloudformation list-stacks --stack-status-filter CREATE_COMPLETE --profile perfil` : lista todas as pilhas em execução
- `aws cloudformation describe-stacks --stack-name id_da_pilha --profile perfil` : verifica o status de uma pilha
- `aws cloudformation detect-stack-drift --stack-name id_da_pilha --profile perfil` : inicia a detecção de desvio para uma pilha
- `aws cloudformation describe-stack-resource-drifts --stack-name stack_drift_detection_id --profile perfil` : verifica o status de desvio de uma pilha usando 'StackDriftDetectionId' do resultado do comando anterior

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cloudformation/
