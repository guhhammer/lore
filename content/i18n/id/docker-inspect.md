# docker inspect

Dapatkan informasi objek-objek Docker secara terperinci.

## Usage
- `docker inspect kontainer|citra|id` : tampilkan informasi suatu kontainer, citra (image), atau volume menggunakan nama atau ID tertentu
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' kontainer` : tampilkan alamat IP suatu kontainer
- `docker inspect -f '\{\{.LogPath\}\}' kontainer` : tampilkan alamat berkas log untuk suatu kontainer
- `docker inspect -f '\{\{.Config.Image\}\}' kontainer` : tampilkan nama citra yang dipakai dalam suatu kontainer
- `docker inspect -f '\{\{json .Config\}\}' kontainer` : tampilkan informasi konfigurasi dalam format JSON
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' kontainer` : tampilkan daftar port yang ditautkan dari sistem host menuju port internal suatu kontainer
- `docker inspect` : tampilkan bantuan

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
