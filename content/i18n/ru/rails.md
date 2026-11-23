# rails

Серверный MVC-фреймворк, написанный на Ruby. Некоторые подкоманды, такие как `generate`, имеют собственную документацию по использованию.

## Usage
- `rails new "имя_проекта"` : создать новый проект Rails
- `rails generate scaffold Post title:string body:text` : сгенерировать каркас для модели Post, задав атрибуты title и body
- `rails db:migrate` : запустить миграции
- `rails routes` : вывести список всех маршрутов
- `rails server` : запустить локальный сервер для текущего проекта на порту 3000
- `rails server -p "порт"` : запустить локальный сервер для текущего проекта на конкретном порту
- `rails console` : открыть консоль для взаимодействия с приложением из командной строки
- `rails -v` : показать версию

## Links
- More information: https://guides.rubyonrails.org/command_line.html
