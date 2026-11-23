# createuser

Create a PostgreSQL user (role).

## Usage
- `createuser --interactive username` : create a user interactively
- `createuser username` : create a user with no special rights
- `createuser -s username` : create a superuser
- `createuser -d -r -P username` : create a user allowed to create databases, manage roles, and prompt for a password
- `createuser -D -R username` : create a user without the ability to create databases or manage roles

## Categories
users

## Links
- More information: https://www.postgresql.org/docs/current/app-createuser.html
- tldr page: https://tldr.inbrowser.app/pages/common/createuser
