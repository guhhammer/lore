# accelerate

Uma biblioteca que habilita o mesmo código PyTorch a rodar em qualquer configuração distribuída.

## Usage
- `accelerate env` : mostra informações do ambiente
- `accelerate config` : cria um arquivo de configuração de forma interativa
- `accelerate estimate-memory nome/modelo` : mostra o custo de memória de GPU estimado para rodar um Modelo de Face aumentado com diferentes tipos de dados
- `accelerate test --config_file caminho/para/config.yaml` : testa um arquivo Accelerate de configuração
- `accelerate launch caminho/para/script.py --cpu` : roda um modelo na CPU com Accelerate
- `accelerate launch caminho/para/script.py --multi_gpu --num_machines 2` : roda um modelo em multi-GPU com Accelerate, com 2 máquinas

## Links
- More information: https://huggingface.co/docs/accelerate/index
