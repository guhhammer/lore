# rails db

Beragam subperintah yang berkaitan dengan database untuk Rauby on Rails.

## Usage
- `rails db:setup` : buat pangkalan data (database) baru, memuat skema dan menginisiasinya dengan data awal
- `rails db` : akses konsol database
- `rails db:create` : buat database yang didefinisikan di environment saat ini
- `rails db:drop` : hapus database yang didefinisikan di environment saat ini
- `rails db:migrate` : jalankan migrasi yang belum
- `rails db:migrate:status` : tampilkan status dari masing-masing file migrasi
- `rails db:rollback` : rollback ke migrasi sebelumnya
- `rails db:seed` : isi database dengan data yang didefinisikan di `db/seeds.rb`

## Links
- More information: https://guides.rubyonrails.org/active_record_migrations.html
