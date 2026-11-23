# get-date

Dapatkan tanggal dan waktu saat ini. Catatan: Perintah ini hanya dapat digunakan dalam PowerShell.

## Usage
- `Get-Date` : tampilkan tanggal dan waktu saat ini
- `Get-Date -Format "yyyy-MM-dd HH:mm:ss"` : tampilkan tanggal dan waktu saat ini dengan format tanggal/waktu .NET
- `(Get-Date).ToUniversalTime()` : tampilkan tanggal dan waktu saat ini dalam zona waktu UTC dan format ISO 8601
- `Get-Date -UnixTimeSeconds 1577836800` : ubah suatu nilai timestamp Unix
