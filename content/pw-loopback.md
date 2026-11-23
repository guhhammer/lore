# pw-loopback

Create loopback devices in PipeWire.

## Usage
- `pw-loopback` : create a loopback device with the default loopback behavior
- `pw-loopback -i 'media.class=Audio/Sink'` : create a loopback device that automatically connects to the speakers
- `pw-loopback -o 'media.class=Audio/Source'` : create a loopback device that automatically connects to the microphone
- `pw-loopback -i 'media.class=Audio/Sink' -o 'media.class=Audio/Source'` : create a dummy loopback device that doesn't automatically connect to anything
- `pw-loopback -m '[FL FR]'` : force the nodes to have specific ports
- `pw-loopback -i 'media.class=Audio/Sink audio.position=[FL FR]' -o 'audio.position=[FR FL]'` : create a loopback device that automatically connects to the speakers and swaps the left and right channels between the sink and source
- `pw-loopback -i 'audio.position=[FR FL]' -o 'media.class=Audio/Source audio.position=[FL FR]'` : create a loopback device that automatically connects to the microphone and swaps the left and right channels between the sink and source
- `pw-loopback -n "node_name"` : give the loopback node a name

## Categories
media, hardware

## Links
- More information: https://docs.pipewire.org/page_man_pw-loopback_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-loopback
