# firejail

Securely sandboxes processes to containers using built-in Linux capabilities.

## Usage
- `sudo firecfg` : integrate firejail with your desktop environment
- `firejail firefox` : open a restricted Mozilla Firefox
- `firejail --net=eth0 --ip=192.168.1.244 /etc/init.d/apache2 start` : start a restricted Apache server on a known interface and address
- `firejail --list` : list running sandboxes
- `firejail --netstats` : list network activity from running sandboxes
- `firejail --shutdown=7777` : shut down a running sandbox
- `firejail --seccomp --private --private-dev --private-tmp --protocol=inet firefox --new-instance --no-remote --safe-mode --private-window` : run a restricted Firefox session to browse the internet
- `firejail --hosts-file=~/myhosts curl http://mysite.arpa` : use custom hosts file (overriding `/etc/hosts` file)

## Categories
network, containers, files

## Links
- More information: https://manned.org/firejail
- tldr page: https://tldr.inbrowser.app/pages/linux/firejail
- Wikipedia: https://en.wikipedia.org/wiki/Firejail
