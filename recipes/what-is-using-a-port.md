# what is using a port

Find the program listening on a port (port 80 here), see what it is, and
stop it if it should not be there.

## Steps
- `sudo ss -tulpn | grep ':80 '` : what listens on port 80, with its program and PID (Linux)
- `sudo lsof -i :80` : the same with lsof, on Linux and macOS
- `sudo fuser -v 80/tcp` : another way to ask, by port (Linux)
- `ps -fp PID` : the whole command line of that PID
- `systemctl status PID` : the service it belongs to, if it is one
- `sudo kill PID` : stop it, once you know it is safe to

## Notes
- Put your port in place of 80, and the number you found in place of PID.
- Only root can listen on ports under 1024, and `ss` and `lsof` show other users' programs only with sudo.
- A service stopped with `kill` may be started again by systemd: stop it with `sudo systemctl stop NAME`.
- macOS: `sudo lsof -nP -iTCP:80 -sTCP:LISTEN`.

## See also
- ss
- lsof
- fuser
- netstat

## Categories
network processes
