# sshfs

Klien sistem berkas (filesystem) berbasis SSH.

## Usage
- `sshfs nama_pengguna@host_jarak_jauh:direktori_jarak_jauh titik_kait` : kaitkan (mount) direktori jarak jauh (remote directory)
- `umount titik_kait` : lepaskan (unmount) direktori jarak jauh (remote directory)
- `sshfs nama_pengguna@host_jarak_jauh:direktori_jarak_jauh -p 2222` : kaitkan direktori jarak jauh (remote directory) dari server dengan port spesifik
- `sshfs nama_pengguna@host_jarak_jauh:direktori_jarak_jauh -C` : gunakan kompresi
- `sshfs -o follow_symlinks nama_pengguna@host_jarak_jauh:direktori_jarak_jauh titik_kait` : ikuti tautan simbolik (symbolic links)

## Links
- More information: https://github.com/libfuse/sshfs/blob/master/sshfs.rst
