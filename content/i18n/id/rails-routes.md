# rails routes

Menampilkan daftar routes di aplikasi Rails.

## Usage
- `rails routes` : menampilkan semua routes
- `rails routes -E` : menampilkan semua routes dengan format yang lebih panjang
- `rails routes -g posts_path|GET|/posts` : menampilkan routes yang sebagian cocok dengan nama helper method URL, HTTP verb, atau path URL
- `rails routes -c posts|Posts|Blogs::PostsController` : menampilkan routes yang memetakan ke controller tertentu

## Links
- More information: https://guides.rubyonrails.org/routing.html
