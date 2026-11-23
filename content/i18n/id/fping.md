# fping

Utilitas ping lebih kuat yang dapat melakukan proses ping pada lebih dari satu host.

## Usage
- `fping 192.168.1.{1..254}` : tampilkan daftar status atas seluruh host pada suatu rentang alamat IP
- `fping -a -g 192.168.1.0/24` : tampilkan daftar seluruh host yang aktif dalam suatu subjaringan menurut definisi network mask (netmask)
- `fping -q -a -g 192.168.1.1 192.168.1.254` : tampilkan daftar seluruh host yang aktif dalam suatu subjaringan berdasarkan rentang alamat IP
- `fping -u -g 192.168.1.0/24` : tampilkan daftar seluruh host yang tidak aktif dalam suatu subjaringan menurut definisi network mask (netmask)

## Links
- More information: https://fping.org/fping.8.html
