# pw-loopback

Ferramenta para Cria dispositivos de loopback no PipeWire.

## Usage
- `pw-loopback` : cria um dispositivo de loopback com o comportamento padrão de loopback
- `pw-loopback -m '[FL FR]' -i 'media.class=Audio/Sink'` : cria um dispositivo de loopback que se conecta automaticamente aos alto-falantes
- `pw-loopback -m '[FL FR]' -o 'media.class=Audio/Source'` : cria um dispositivo de loopback que se conecta automaticamente ao microfone
- `pw-loopback -m '[FL FR]' -i 'media.class=Audio/Sink' -o 'media.class=Audio/Source'` : cria um dispositivo fictício que não se conecta automaticamente a nada
- `pw-loopback -i 'media.class=Audio/Sink audio.position=[FL FR]' -o 'audio.position=[FR FL]'` : cria um dispositivo de loopback que se conecta automaticamente aos alto-falantes e troca os canais esquerdo e direito entre o dispositivo de entrada e o de saída
- `pw-loopback -i 'audio.position=[FR FL]' -o 'media.class=Audio/Source audio.position=[FL FR]'` : cria um dispositivo de loopback que se conecta automaticamente ao microfone e troca os canais esquerdo e direito entre o dispositivo de entrada e o de saída

## Links
- More information: https://docs.pipewire.org/page_man_pw-loopback_1.html
