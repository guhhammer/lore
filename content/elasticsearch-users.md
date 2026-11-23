# elasticsearch-users

Manage native realm users in Elasticsearch, including creating, updating, and deleting users.

## Usage
- `elasticsearch-users useradd username` : add a new user interactively (prompts for password)
- `elasticsearch-users useradd username -r role1,role2` : add a new user and specify roles
- `elasticsearch-users passwd username` : change the password for an existing user
- `elasticsearch-users userdel username` : delete a user
- `elasticsearch-users list` : list all users in the native realm

## Categories
users

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/elasticsearch-users
