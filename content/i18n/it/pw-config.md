# pw-config

Elenca i percorsi di configurazione e le sezioni che verranno usate dal server PipeWire e dai client.

## Usage
- `pw-config` : elenca tutti i file di configurazione che verranno usati
- `pw-config -n pipewire-pulse.conf` : elenca tutti i file di configurazione che verranno usati dal server PipeWire PulseAudio
- `pw-config -n pipewire-pulse.conf list` : elenca tutte le sezioni di configurazione usate dal server PipeWire PulseAudio
- `pw-config -n jack.conf list context.properties` : elenca i frammenti `context.properties` usati dai client JACK
- `pw-config -n jack.conf merge context.properties` : elenca il `context.properties` unificato usato dai client JACK
- `pw-config -n pipewire.conf -r merge context.modules` : elenca i `context.modules` unificati usati dal server PipeWire e riformatta
- `pw-config -h` : mostra l'aiuto

## Links
- More information: https://docs.pipewire.org/page_man_pw-config_1.html
