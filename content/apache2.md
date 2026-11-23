# apache2

Apache Hypertext Transfer Protocol Server.

## Usage
- `apache2 -d serverroot` : set the initial value for the ServerRoot directive to serverroot
- `apache2 -f config` : uses the directives in the file config on startup
- `apache2 -k start|restart|graceful|stop|graceful-stop` : signals apache2 to start, restart, or stop
- `apache2 -C directive` : process the configuration directive before reading config files
