# ld

Hubungkan berkas-berkas obyek berbarengan untuk membangun suatu berkas program executable.

## Usage
- `ld jalan/menuju/berkas.o -o jalan/menuju/output_executable` : hubungkan berkas obyek tertentu yang tidak memiliki kebergantungan ke suatu executable
- `ld jalan/menuju/berkas1.o jalan/menuju/berkas2.o -o jalan/menuju/output_executable` : hubungkan dua berkas obyek berbarengan
- `ld -o jalan/menuju/output_executable -I /lib/ld-linux-x86-64.so.2 /lib/crt1.o /lib/crti.o -lc jalan/menuju/berkas.o /lib/crtn.o` : hubungkan secara dinamis sebuah program x86_64 ke glibc (jalan-jalan berkas berubah bergantung pada sistem)

## Links
- More information: https://sourceware.org/binutils/docs/ld.html#Options
