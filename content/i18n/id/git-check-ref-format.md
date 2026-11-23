# git check-ref-format

Cek apakah nama suatu referensi (refname) sesuai syarat, dan keluar dengan nilai status di luar angka nol jika tidak.

## Usage
- `git check-ref-format refs/head/refname` : cek kesesuaian format suatu nama referensi
- `git check-ref-format --branch @{-1}` : cetak nama cabang yang terakhir kali diperiksa sebelum mengganti ke cabang saat ini
- `git check-ref-format --normalize refs/head/refname` : lakukan normalisasi terhadap nama referensi

## Links
- More information: https://git-scm.com/docs/git-check-ref-format
