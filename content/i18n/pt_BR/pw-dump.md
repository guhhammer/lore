# pw-dump

Exibe o estado atual do PipeWire como JSON, incluindo as informações sobre nós, dispositivos, módulos, portas e outros objetos.

## Usage
- `pw-dump` : exibe uma representação em JSON do estado atual da instância padrão do PipeWire
- `pw-dump --monitor` : exibe o estado atual monitorando mudanças, exibindo-as novamente
- `pw-dump --remote nome_do_remoto >caminho/para/arquivo.json` : salva o estado atual de uma instância remota para um arquivo
- `pw-dump --color never|always|auto` : define uma configuração de Cor
- `pw-dump --help` : exibir ajuda

## Links
- More information: https://docs.pipewire.org/page_man_pw-dump_1.html
