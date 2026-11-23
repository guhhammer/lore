# input

Kirim sinyal input terhadap sebuah perangkat Android. Perintah ini hanya dapat dijalankan melalui `adb shell`.

## Usage
- `input keyevent kode_event` : masukkan input karakter (layaknya pada papan kunci / keyboard) terhadap perangkat Android
- `input text "teks"` : masukkan input teks ke dalam perangkat Android (spasi ditandai dengan `%s`)
- `input tap posisi_x posisi_y` : masukkan input sentuhan layar pada posisi tertentu
- `input swipe posisi_awal_x posisi_awal_y posisi_akhir_x posisi_akhir_y durasi_dalam_milidetik` : simulasikan gerakan usap/swipe terhadap perangkat Android
- `input swipe posisi_x posisi_y posisi_x posisi_y durasi_dalam_milidetik` : simulasikan interaksi tekan-dan-tahan terhadap perangkat Android
