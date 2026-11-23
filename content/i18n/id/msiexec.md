# msiexec

Pasang, perbarui, perbaiki, atau hapus program Windows melalui berkas MSI dan MSP yang tersedia.

## Usage
- `msiexec /package jalan\menuju\berkas.msi` : pasang suatu program melalui berkas MSI
- `msiexec /package https://example.com/installer.msi` : pasang berkas MSI dari suatu situs web
- `msiexec /update jalan\menuju\berkas.msp` : pasang pembaruan suatu program melalui suatu berkas MSP
- `msiexec /uninstall jalan\menuju\berkas` : menghapus pemasangan atau pembaruan suatu program melalui file MSI atau MSP yang tersedia
