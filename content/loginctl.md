# loginctl

Manage the systemd login manager.

## Usage
- `loginctl` : print all current sessions
- `loginctl show-session session_id -a` : print all properties of a specific session
- `loginctl show-user username` : print all properties of a specific user
- `loginctl show-user username -p property_name` : print a specific property of a user
- `loginctl list-users -H hostname` : execute a `loginctl` operation on a remote host
- `loginctl terminate-user username` : log a user out on all of their sessions
- `loginctl -h` : display help

## See also
- lslogins

## Categories
users, documents, system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/loginctl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/loginctl
