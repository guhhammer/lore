# aws ec2

Inteface de linha de comando para o AWS EC2. Provê capacidade computacional segura e flexível na nuvem da AWS para proporcionar um desenvolvimento e subida para produção de aplicações rapidamente.

## Usage
- `aws ec2 describe-instances --instance-ids id_da_instância` : exibe informações sobre uma insntância específica
- `aws ec2 describe-instances` : exibe informações sobre todas as instâncias
- `aws ec2 describe-volumes` : exibe informações sobre todos os volumes EC2
- `aws ec2 delete-volume --volume-id id_do_volume` : deleta um volume EC2
- `aws ec2 create-snapshot --volume-id id_do_volume` : cria um snapshot de um volume EC2
- `aws ec2 describe-images` : lista as AMIs (Imagem de Máquina da Amazon) disponíveis
- `aws ec2 help` : lista todos os comandos EC2 disponíveis
- `aws ec2 subcomando help` : exibe ajuda específica para um subcomando da EC2

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ec2/
