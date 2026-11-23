# accelerate

Sebuah pustaka/library yang memungkinkan kode PyTorch yang sama dapat dijalankan secara menyebar.

## Usage
- `accelerate env` : tampilkan informasi lingkungan proyek PyTorch saat ini
- `accelerate config` : buat file konfigurasi secara interaktif
- `accelerate estimate-memory nama/model` : tampilkan prakiraan kapasitas memori GPU yang dibutuhkan untuk menjalankan model Hugging Face dengan tipe data yang berbeda
- `accelerate test --config_file jalan/menuju/config.yaml` : uji validitas sebuah file konfigurasi Accelerate
- `accelerate launch jalan/menuju/script.py --cpu` : jalankan sebuah model PyTorch dengan Accelerate, menggunakan CPU saja
- `accelerate launch jalan/menuju/script.py --multi_gpu --num_machines 2` : jalankan model dengan Accelerate, menggunakan GPU dari 2 perangkat yang berbeda

## Links
- More information: https://huggingface.co/docs/accelerate/index
