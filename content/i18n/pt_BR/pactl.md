# pactl

Controla um servidor PulseAudio em execução.

## Usage
- `pactl info` : mostra informações sobre o servidor de áudio
- `pactl list sinks short` : lista todas as sinks (ou outros tipos - sinks são saídas e sink-inputs são fluxos de áudio ativos)
- `pactl set-default-sink 1` : altera a sink padrão (saída) para 1 (o número pode ser obtido através do subcomando `list`)
- `pactl move-sink-input 627 1` : move a sink-input 627 para a sink 1
- `pactl set-sink-volume 1 0.75` : define o volume da sink 1 para 75%
- `pactl set-sink-mute @DEFAULT_SINK@ toggle` : ativa/Desativa o modo silencioso na sink padrão (usando o nome especial `@DEFAULT_SINK@`)

## Links
- More information: https://manned.org/pactl
