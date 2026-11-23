# pkg_delete

Hapus pemasangan paket dalam OpenBSD.

## Usage
- `pkg_delete paket` : hapus sebuah paket yang terpasang
- `pkg_delete -a paket` : hapus paket beserta penunjang (dependency) yang juga tidak lagi dipakai
- `pkg_delete -n paket` : tampilkan apa yang terjadi ketika menghapus suatu paket tanpa memprosesnya secara langsung (dry-run)

## Links
- More information: https://man.openbsd.org/pkg_delete
