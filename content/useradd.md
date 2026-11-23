# useradd

Create a new user.

## Usage
- `sudo useradd username` : create a new user
- `sudo useradd -u user_id username` : create a new user with the specified user ID
- `sudo useradd -s path/to/shell username` : create a new user with the specified shell
- `sudo useradd -G group1,group2,... username` : create a new user belonging to additional groups (mind the lack of whitespace)
- `sudo useradd -m username` : create a new user with the default home directory
- `sudo useradd -k path/to/template_directory -m username` : create a new user with the home directory filled by template directory files
- `sudo useradd -r username` : create a new system user without the home directory

## See also
- users
- userdel
- usermod

## Categories
users

## Links
- More information: https://manned.org/useradd
- tldr page: https://tldr.inbrowser.app/pages/linux/useradd
