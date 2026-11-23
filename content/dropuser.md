# dropuser

Remove an existing PostgreSQL user (role).

## Usage
- `dropuser -i` : prompt for confirmation and the username before user removal
- `dropuser username` : remove user instantly
- `dropuser --if-exists username` : do not throw an error if the user to be removed doesn't exist
- `dropuser -h 127.0.0.1 -p 4321 username` : remove a user on the server with address 127.0.0.1 on port 4321
- `dropuser -U admin -h 127.0.0.1 -p 4321 username` : remove a user on the server with address 127.0.0.1 on port 4321 as user "admin"

## Categories
users, network

## Links
- More information: https://www.postgresql.org/docs/current/app-dropuser.html
- tldr page: https://tldr.inbrowser.app/pages/common/dropuser
