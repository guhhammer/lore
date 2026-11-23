# sshd

Secure Shell Daemon - mengizinkan mesin jarak jauh (remote host) untuk masuk (log in) secara aman ke mesin klien. Mesin jarak jauh dapat mengeksekusi perintah di mesin ini.

## Usage
- `sshd` : mulai daemon di latar belakang (background)
- `sshd -D` : jalankan sshd di latar depan (foreground)
- `sshd -D -d` : jalankan dengan output verbose (untuk proses debug)
- `sshd -p port` : jalankan pada port tertentu

## Links
- More information: https://man.openbsd.org/sshd
