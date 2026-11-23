# rails db

Управлять базами данных в Ruby on Rails.

## Usage
- `rails db:setup` : создать базы данных, загрузить схему и заполнить начальными данными
- `rails db` : открыть консоль базы данных
- `rails db:create` : создать базы данных, определённые в текущем окружении
- `rails db:drop` : удалить базы данных, определённые в текущем окружении
- `rails db:migrate` : запустить ожидающие миграции
- `rails db:migrate:status` : показать статус каждого файла миграции
- `rails db:rollback` : откатить последнюю миграцию
- `rails db:seed` : заполнить текущую базу данных данными, определёнными в `db/seeds.rb`

## Links
- More information: https://guides.rubyonrails.org/active_record_migrations.html
