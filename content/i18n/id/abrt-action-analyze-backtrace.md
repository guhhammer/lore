# abrt-action-analyze-backtrace

Analisis backtrace C/C++. Hasilkan hash duplikasi, peringkat backtrace (backtrace rating), dan identifikasi fungsi crash. Simpan data sebagai elemen baru `duphash`, `rating`, `crash_function` di dalam direktori masalah.

## Usage
- `abrt-action-analyze-backtrace` : lakukan analisis backtrace untuk direktori kerja (working directory) saat ini
- `abrt-action-analyze-backtrace -d jalan/menuju/direktori` : lakukan analisis backtrace untuk direktori tertentu
- `abrt-action-analyze-backtrace -v` : lakukan analisis backtrace secara verbose

## Links
- More information: https://manned.org/abrt-action-analyze-backtrace
