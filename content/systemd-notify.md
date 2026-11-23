# systemd-notify

Notify the service manager about start-up completion and other daemon status changes. This command is useless outside systemd service scripts.

## Usage
- `systemd-notify --booted` : notify systemd that the service has completed its initialization and is fully started. It should be invoked when the service is ready to accept incoming requests
- `systemd-notify --ready` : signal to systemd that the service is ready to handle incoming connections or perform its tasks
- `systemd-notify --status "Add custom status message here..."` : provide a custom status message to systemd (this information is shown by `systemctl status`)

## Categories
system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-notify.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-notify
