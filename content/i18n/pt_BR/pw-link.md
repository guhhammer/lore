# pw-link

Gerenciar conexões entre portas no PipeWire.

## Usage
- `pw-link -oiI` : lista todos as saídas e entradas de áudio com seus IDs
- `pw-link output_port_name input_port_name` : cria uma conexão entre uma porta de entrada e uma porta de saída
- `pw-link -d output_port_name input_port_name` : desconecta duas portas
- `pw-link -lI` : lista todas as conexões com seus IDs
- `pw-link -h` : mostra ajuda

## Links
- More information: https://gitlab.freedesktop.org/pipewire/pipewire/-/wikis/Virtual-Devices
