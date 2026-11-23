# rails generate

Генерировать новые шаблоны Rails в существующем проекте.

## Usage
- `rails generate` : вывести список всех доступных генераторов
- `rails generate model Post title:string body:text` : сгенерировать новую модель Post с атрибутами title и body
- `rails generate controller Posts index show new create` : сгенерировать новый контроллер Posts с действиями index, show, new и create
- `rails generate migration AddCategoryToPost category:string` : сгенерировать новую миграцию, добавляющую атрибут category в существующую модель Post
- `rails generate scaffold Post title:string body:text` : сгенерировать каркас для модели Post, задав атрибуты title и body

## Links
- More information: https://guides.rubyonrails.org/command_line.html#bin-rails-generate
