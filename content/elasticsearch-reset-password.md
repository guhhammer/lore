# elasticsearch-reset-password

Reset the passwords of users in the native realm and built-in users.

## Usage
- `elasticsearch-reset-password -u user` : reset the password of the user to an auto-generated value and print it in the console
- `elasticsearch-reset-password -u user -i` : prompt interactively to reset the password for a native user
- `elasticsearch-reset-password --url host:port -u user -i` : interactively reset the password for a user at a specified Elasticsearch node URL

## Categories
users, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/elasticsearch-reset-password
