# run commands on a server over ssh

Run one command or a whole local script on another machine, without logging in by hand.

## Steps
- `ssh NAME@host 'uptime'` : one command; its output comes back here
- `ssh NAME@host 'cd /app && git pull && sudo systemctl restart app'` : several, in order
- `ssh NAME@host 'bash -s' < local-script.sh` : run a script from this machine there
- `ssh -t NAME@host 'sudo systemctl restart app'` : one that asks for a password (-t gives it a terminal)
- `ssh NAME@host 'cat /var/log/app.log' > app.log` : bring a file's text back

## See also
- ssh

## Categories
network
