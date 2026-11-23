# pwsh

Sebuah syel/shell dan bahasa pemrograman berbasis naskah/script yang dirancang untuk administrasi sistem komputer. Perintah ini merujuk kepada PowerShell versi 6 ke atas (juga dikenal sebagai PowerShell Core dan PowerShell lintas sistem operasi). Gunakan perintah `powershell` daripada `pwsh` untuk menggunakan PowerShell versi bawaan Windows (5.1 ke bawah).

## Usage
- `pwsh` : jalankan sesi PowerShell interaktif baru
- `pwsh -NoProfile` : jalankan sesi interaktif tanpa memuat profil konfigurasi startup
- `pwsh -Command "echo 'powershell telah dieksekusi'"` : jalankan kumpulan perintah secara spesifik
- `pwsh -File jalan/menuju/naskah.ps1` : jalankan suatu naskah/skrip perintah PowerShell
- `pwsh -Version versi` : jalankan suatu sesi dengan versi PowerShell tertentu
- `pwsh -NoExit` : cegah sesi PowerShell dari keluar secara otomatis setelah menjalankan perintah startup
- `pwsh -InputFormat Text|XML` : tentukan format data yang akan dimasukkan ke dalam PowerShell
- `pwsh -OutputFormat Text|XML` : tentukan format data yang ingin dikeluarkan dari perintah-perintah PowerShell
