# rabbitmqctl-users

Управлять пользователями RabbitMQ, их разрешениями и тегами.

## Usage
- `rabbitmqctl list_users` : вывести список всех пользователей
- `rabbitmqctl add_user имя_пользователя пароль` : добавить нового пользователя с паролем
- `rabbitmqctl delete_user имя_пользователя` : удалить существующего пользователя
- `rabbitmqctl change_password имя_пользователя новый_пароль` : изменить пароль пользователя
- `rabbitmqctl set_permissions -p виртуальный_хост имя_пользователя чтение запись настройка` : установить разрешения для пользователя на конкретном виртуальном хосте
- `rabbitmqctl clear_permissions -p виртуальный_хост имя_пользователя` : очистить все разрешения для пользователя на конкретном виртуальном хосте
- `rabbitmqctl set_user_tags имя_пользователя тег1 тег2 ...` : назначить один или несколько тегов (например, administrator) пользователю

## Links
- More information: https://www.rabbitmq.com/docs/management
