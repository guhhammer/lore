# bun audit

Periksa potensi celah keamanan dalam kumpulan paket yang terpasang.

## Usage
- `bun audit` : lakukan audit atas seluruh proyek dengan berkas ketergantungan paket `bun.lock`
- `bun audit --audit-level low|moderate|high|critical` : hanya tampilkan celah keamanan dengan tingkat keparahan setara atau lebih
- `bun audit --prod` : hanya audit kumpulan paket yang diperlukan untuk dijalankan dalam lingkungan laik produksi
- `bun audit --ignore CVE-XXXX-YYYY` : abaikan celah keamanan dengan kode CVE tertentu
- `bun audit --json` : luarkan laporan audit dalam format JSON

## Links
- More information: https://bun.com/docs/pm/cli/audit
