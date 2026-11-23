# rabbitmqctl-users

Manage RabbitMQ users, their permissions, and tags.

## Usage
- `rabbitmqctl list_users` : list all users
- `rabbitmqctl add_user username password` : add a new user with a password
- `rabbitmqctl delete_user username` : delete an existing user
- `rabbitmqctl change_password username new_password` : change the password for a user
- `rabbitmqctl set_permissions -p vhost username read write configure` : set permissions for a user on a specific virtual host
- `rabbitmqctl clear_permissions -p vhost username` : clear all permissions for a user on a specific virtual host
- `rabbitmqctl set_user_tags username tag1 tag2 ...` : assign one or more tags (e.g., administrator) to a user

## Categories
users, network

## Links
- More information: https://www.rabbitmq.com/docs/management
- tldr page: https://tldr.inbrowser.app/pages/common/rabbitmqctl-users
