# pw-loopback

Crea dispositivi loopback in PipeWire.

## Usage
- `pw-loopback` : crea un dispositivo loopback con comportamento predefinito
- `pw-loopback -i 'media.class=Audio/Sink'` : crea un dispositivo loopback che si connette automaticamente agli altoparlanti
- `pw-loopback -o 'media.class=Audio/Source'` : crea un dispositivo loopback che si connette automaticamente al microfono
- `pw-loopback -i 'media.class=Audio/Sink' -o 'media.class=Audio/Source'` : crea un dispositivo loopback fittizio che non si connette automaticamente a nulla
- `pw-loopback -m '[FL FR]'` : forza i nodi ad avere porte specifiche
- `pw-loopback -i 'media.class=Audio/Sink audio.position=[FL FR]' -o 'audio.position=[FR FL]'` : crea un dispositivo loopback che si connette automaticamente agli altoparlanti e scambia i canali sinistro e destro tra sink e source
- `pw-loopback -i 'audio.position=[FR FL]' -o 'media.class=Audio/Source audio.position=[FL FR]'` : crea un dispositivo loopback che si connette automaticamente al microfono e scambia i canali sinistro e destro tra sink e source
- `pw-loopback -n "nome_nodo"` : dai un nome al nodo loopback

## Links
- More information: https://docs.pipewire.org/page_man_pw-loopback_1.html
