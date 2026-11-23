# avrdude

Program driver untuk memrogram mikrokontroler Atmel AVR.

## Usage
- `avrdude -p part_id -c programmer_id -U flash:r:file.hex:i` : baca (read) isi penyimpanann flash ROM suatu perangkat mikrokontroler AVR dengan nomor suku cadang (part)
- `avrdude -p part_id -c programmer -U flash:w:file.hex` : tulis (write) isi memori menuju penyimpanan flash perangkat mikrokontroler AVR
- `avrdude -p \?` : tampilkan daftar perangkat AVR yang didukung
- `avrdude -c \?` : tampilkan daftar pemrogram AVR yang didukung
