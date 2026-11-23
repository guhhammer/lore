# dbus-daemon

The D-Bus message daemon, allowing multiple programs to exchange messages.

## Usage
- `dbus-daemon --config-file path/to/file` : run the daemon with a configuration file
- `dbus-daemon --session` : run the daemon with the standard per-login-session message bus configuration
- `dbus-daemon --system` : run the daemon with the standard systemwide message bus configuration
- `dbus-daemon --address address` : set the address to listen on and override the configuration value for it
- `dbus-daemon --print-pid` : output the process ID to `stdout`
- `dbus-daemon --syslog` : force the message bus to write to the system log for messages

## Categories
system, files

## Links
- More information: https://dbus.freedesktop.org/doc/dbus-daemon.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dbus-daemon
