# chpasswd

Change the passwords for multiple users by using `stdin`.

## Usage
- `printf "username:new_password" | sudo chpasswd` : change the password for a specific user
- `printf "username_1:new_password_1\nusername_2:new_password_2" | sudo chpasswd` : change the passwords for multiple users (The input text must not contain any spaces.)
- `printf "username:new_encrypted_password" | sudo chpasswd -e` : change the password for a specific user, and specify it in encrypted form
- `printf "username:new_password" | sudo chpasswd -c NONE|DES|MD5|SHA256|SHA512` : change the password for a specific user, and use a specific encryption for the stored password

## See also
- passwd

## Categories
users

## Links
- More information: https://manned.org/chpasswd
- tldr page: https://tldr.inbrowser.app/pages/linux/chpasswd
