# ssh-add

Mengelola kunci SSH yang dimuat di dalam `ssh-agent`. Pastikan `ssh-agent` sudah berjalan agar kunci dapat dimuat ke dalamnya.

## Usage
- `ssh-add` : tambahkan kunci SSH default di `~/.ssh` ke ssh-agent
- `ssh-add lokasi/ke/kunci_privat` : tambahkan kunci spesifik ke ssh-agent
- `ssh-add -l` : tampilkan sidik jari (fingerprint) dari kunci yang sedang dimuat
- `ssh-add -d lokasi/ke/kunci_privat` : hapus sebuah kunci dari ssh-agent
- `ssh-add -D` : hapus semua kunci yang sedang dimuat dari ssh-agent
- `ssh-add -K lokasi/ke/kunci_privat` : tambahkan kunci ke ssh-agent dan keychain

## Links
- More information: https://man.openbsd.org/ssh-add
