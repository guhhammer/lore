# alembic

Alat migrasi pangkalan data untuk SQLAlchemy.

## Usage
- `alembic init jalan/menuju/direktori` : inisialisasikan Alembic dalam suatu proyek
- `alembic revision --autogenerate -m "pesan"` : buatu sebuah berkas skrip migrasi dengan fitur generasi migrasi otomatis
- `alembic upgrade head` : lakukan proses pemutakhiran pangkalan data menuju struktur terkini
- `alembic downgrade -1` : turunkan pangkalan data menuju satu revisi sebelumnya
- `alembic history` : tampilkan daftar riwayat migrasi

## Links
- More information: https://manned.org/alembic
