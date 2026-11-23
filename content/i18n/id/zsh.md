# zsh

Z SHell, suatu penerjemah baris perintah yang kompatibel dengan standar bahasa Bash.

## Usage
- `zsh` : jalankan sesi interaktif baru
- `zsh -c "echo 'perintah zsh telah dijalankan'"` : jalankan kumpulan perintah secara spesifik
- `zsh jalan/menuju/naskah.zsh` : jalankan suatu naskah/skrip perintah
- `zsh -n jalan/menuju/naskah.zsh` : periksa suatu naskah/skrip perintah untuk kesalahan sintaks tanpa menjalankan isi perintahnya
- `echo "echo 'perintah zsh telah dijalankan'" | zsh` : baca dan jalankan kumpulan perintah dari `stdin`
- `zsh -x jalan/menuju/naskah.zsh` : jalankan suatu naskah/skrip, dan tampilkan isi masing-masing perintah sebelum menjalankannya
- `zsh -v` : jalankan sesi interaktif baru dengan mengeluarkan kembali isi perintah masukan sebelum menjalankannya
- `zsh -f` : jalankan sesi interaktif baru tanpa memuat perintah konfigurasi awal tingkat pengguna (misalnya dari `~/.zshrc`)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Invocation.html#Invocation
