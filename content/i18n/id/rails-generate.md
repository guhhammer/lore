# rails generate

Membuat Rails templates yang baru ke suatu proyek.

## Usage
- `rails generate` : menampilkan semua generator yang tersedia
- `rails generate model Post judul:string uraian:text` : membuat model baru bernama Post dengan atribut judul dan uraian
- `rails generate controller Posts index show new create` : mmebuat controller baru bernama Posts dengan actions index, show, new dan create
- `rails generate migration AddKategoriToPost kategori:string` : membuat migrasi baru yang menambahkan atribut kategori ke model yang sudah ada bernama Post
- `rails generate scaffold Post title:string body:text` : membuat scaffold untuk model bernama Post, dengan pendefinisian atribut judul dan uraian

## Links
- More information: https://guides.rubyonrails.org/command_line.html#bin-rails-generate
