# rsh

Jalankan kumpulan perintah pada suatu host jarak jauh.

## Usage
- `rsh host_jarak_jauh ls -l` : jalankan sebuah perintah pada suatu host jarak jauh
- `rsh host_jarak_jauh -l nama_pengguna ls -l` : jalankan perintah pada host jarak jauh dengan nama pengguna (username) tertentu
- `rsh host_jarak_jauh --no-err ls -l` : alihkan `stdin` menuju `/dev/null` saat menjalankan perintah pada host jarak jauh

## Links
- More information: https://www.gnu.org/software/inetutils/manual/inetutils.html#rsh-invocation
