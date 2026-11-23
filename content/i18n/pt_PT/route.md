# route

Alteração manual da tabela de rotas. Necessita de root.

## Usage
- `sudo route add "endereco_ip_destino" "endereco_gateway"` : adiciona uma rota para um destino passando por um gateway
- `sudo route add "endereco_ip_subnet/24" "endereco_gateway"` : adiciona uma rota para um rede /24 passando por um gateway
- `sudo route -t add "endereco_ip_destino/24" "endereco_gateway"` : corre em modo de teste (não realiza alterações, apenas a mostra)
- `sudo route flush` : remove todas as rotas
- `sudo route delete "endereco_ip_destino/24"` : remove uma rota especifica
- `sudo route get "destino"` : procura e mostra a rota para um destino (nome da máquina ou endereço IP)

## Links
- More information: https://keith.github.io/xcode-man-pages/route.8.html
