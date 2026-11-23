# clear-history

Hapus kumpulan entri riwayat perintah dari PowerShell. Catatan: Perintah `clhy` dapat digunakan sebagai alias untuk `Clear-History`.

## Usage
- `Clear-History` : hapus seluruh riwayat perintah dari sesi PowerShell saat ini
- `Clear-History -CommandLine "perintah"` : hapus riwayat berdasarkan nama perintah tertentu
- `Clear-History -CommandLine "perintah1", "perintah2", ...` : hapus riwayat berdasarkan berbagai nama perintah
- `Clear-History -Id nomor_induk` : hapus entri riwayat berdasarkan nomor induk (ID)
- `Clear-History -Id nomor_induk1, nomor_induk2, ...` : hapus entri riwayat berdasarkan beberapa nomor induk
- `Clear-History -Id (nomor_induk_awal..nomor_induk_akhir)` : hapus riwayat berdasarkan rentang nomor induk
- `Clear-History -WhatIf` : tampilkan riwayat mana saja yang akan dihapus, tanpa melakukannya (dry-run)
- `Clear-History -Confirm` : tampilkan permintaan konfirmasi sebelum menghapus riwayat
