# update-inetd

Create, remove, enable or disable entry /etc/inetd.conf.

## Usage
- `update-inetd [option...] command argument` : the general form, as the manual gives it
- `update-inetd --add entry-line` : add  an entry to /etc/inetd.conf
- `update-inetd --remove entry-regex` : remove an entry-line matching entry-regex (e.g
- `update-inetd --group groupname` : specify that the new entry should be placed in group groupname (e.g
- `update-inetd --pattern pattern` : this option can be used to select a service

## Categories
text
