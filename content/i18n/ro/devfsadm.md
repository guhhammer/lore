# devfsadm

Comandă de administrare pentru `/dev`. Menține spațiul de nume `/dev`.

## Usage
- `devfsadm -c disk` : scanează după discuri noi
- `devfsadm -C -v` : curăță orice legături (links) `/dev` invalide și scanează după dispozitive noi
- `devfsadm -C -v -n` : simulare - afișează ce s-ar schimba, dar nu efectuează nicio modificare

## Links
- More information: https://www.unix.com/man-page/sunos/1m/devfsadm
