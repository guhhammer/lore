# nologin

Alternative shell that prevents a user from logging in.

## Usage
- `chsh -s user nologin` : set a user's login shell to `nologin` to prevent the user from logging in
- `echo "declined_login_message" >/etc/nologin.txt` : customize message for users with the login shell of `nologin`

## Categories
users, shell

## Links
- More information: https://manned.org/nologin.8
- tldr page: https://tldr.inbrowser.app/pages/linux/nologin
