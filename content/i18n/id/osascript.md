# osascript

Jalankan AppleScript atau JavaScript for Automation (JXA) dari command-line.

## Usage
- `osascript -e 'say "Halo dunia"'` : jalankan sebuah perintah AppleScript
- `osascript -e 'say "Halo"' -e 'say "dunia"'` : jalankan beberapa perintah AppleScript
- `osascript jalan/menuju/apple.scpt` : mengeksekusi perintah dari file AppleScript yang telah terkompilasi (`*.scpt`), terbundel (`*.scptd`), atau secara plaintext (`*.applescript`)
- `osascript -e 'id of app "Aplikasi"'` : mendapatkan ID pengenal (bundle identifier) dari sebuah aplikasi (dapat digunakan untuk perintah `open -b`)
- `osascript -l JavaScript -e "console.log('Halo dunia');"` : jalankan sebuah perintah JavaScript
- `osascript -l JavaScript jalan/menuju/javascript.js` : mengeksekusi perintah dari file JavaScript

## Links
- More information: https://keith.github.io/xcode-man-pages/osascript.1.html
