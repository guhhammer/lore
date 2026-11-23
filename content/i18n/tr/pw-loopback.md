# pw-loopback

PipeWire'da geri döngü cihazları yaratma aracı.

## Usage
- `pw-loopback` : varsayılan geri döngü davranışına sahip bir geri döngü cihazı yarat
- `pw-loopback -m '[FL FR]' -i 'media.class=Audio/Sink'` : hoparlörlere otomatik olarak bağlanan bir geri döngü cihazı yarat
- `pw-loopback -m '[FL FR]' -o 'media.class=Audio/Source'` : mikrofona otomatik olarak bağlanan bir geri döngü cihazı yarat
- `pw-loopback -m '[FL FR]' -i 'media.class=Audio/Sink' -o 'media.class=Audio/Source'` : hiçbir şeye otomatik olarak bağlanmayan salak bir geri döngü cihazı yarat
- `pw-loopback -i 'media.class=Audio/Sink audio.position=[FL FR]' -o 'audio.position=[FR FL]'` : hoparlörlere otomatik olarak bağlanan ve taban-kaynak arasında sağ-sol kanalların yerini değiştiren bir geri döngü cihazı yarat
- `pw-loopback -i 'audio.position=[FR FL]' -o 'media.class=Audio/Source audio.position=[FL FR]'` : mikrofona otomatik olarak bağlanan ve taban-kaynak arasında sağ-sol kanalların yerini değiştiren bir geri döngü cihazı yarat

## Links
- More information: https://docs.pipewire.org/page_man_pw-loopback_1.html
