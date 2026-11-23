# alien

Konversikan berbagai paket instalasi ke format lain.

## Usage
- `sudo alien -d jalan/menuju/berkas` : konversikan file instalasi tertentu ke format Debian (ekstensi `.deb`)
- `sudo alien -r jalan/menuju/berkas` : konversikan file instalasi tertentu ke format Red Hat (ekstensi `.rpm`)
- `sudo alien -t jalan/menuju/berkas` : konversikan file instalasi tertentu ke file instalasi Slackware (ekstensi `.tgz`)
- `sudo alien -d -i jalan/menuju/berkas` : konversikan file instalasi tertentu ke format Debian dan install di sistem

## Links
- More information: https://manned.org/alien
