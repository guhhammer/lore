# last

View the last logged in users.

## Usage
- `last` : view last login information (e.g., username, terminal, boot time, kernel) of all users as read from `/var/log/wtmp`
- `last username` : list login information of a specific user
- `last -n login_count` : specify how many of the last logins to show
- `last -F -a` : print the full date and time for entries and then display the hostname column last to prevent truncation
- `last username -i` : view all logins by a specific user and show the IP address instead of the hostname
- `last -s -7days` : list information since a specific time and date
- `last reboot` : view all recorded reboots (i.e., the last logins of the pseudo user "reboot")
- `last -h` : display help

## Categories
users, system, network

## Links
- More information: https://manned.org/last
- tldr page: https://tldr.inbrowser.app/pages/linux/last
