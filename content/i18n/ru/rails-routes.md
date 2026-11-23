# rails routes

Выводить список маршрутов в приложении Rails.

## Usage
- `rails routes` : вывести список всех маршрутов
- `rails routes -E` : вывести список всех маршрутов в расширенном формате
- `rails routes -g posts_path|GET|/posts` : вывести список маршрутов, частично совпадающих с именем URL-хелпера, HTTP-методом или URL-путём
- `rails routes -c posts|Posts|Blogs::PostsController` : вывести список маршрутов, связанных с конкретным контроллером

## Links
- More information: https://guides.rubyonrails.org/routing.html
