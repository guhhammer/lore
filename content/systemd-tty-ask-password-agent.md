# systemd-tty-ask-password-agent

List or process pending systemd password requests.

## Usage
- `systemd-tty-ask-password-agent --list` : list all currently pending system password requests
- `systemd-tty-ask-password-agent --watch` : continuously process password requests
- `systemd-tty-ask-password-agent --query` : process all currently pending system password requests by querying the user on the calling TTY
- `systemd-tty-ask-password-agent --wall` : forward password requests to wall instead of querying the user on the calling TTY

## Categories
users, system, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-tty-ask-password-agent
