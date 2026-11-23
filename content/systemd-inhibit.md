# systemd-inhibit

Prohibit the system from entering certain power states. Inhibitor locks may be used to block or delay system sleep and shutdown requests as well as automatic idle handling.

## Usage
- `systemd-inhibit` : list all active inhibition locks and the reasons for their creation
- `systemd-inhibit --what shutdown sleep 5` : block system shutdown for a specified number of seconds with the `sleep` command
- `systemd-inhibit --what sleep:idle wget https://example.com/file` : keep the system from sleeping or idling until the download is complete
- `systemd-inhibit --what sleep:handle-lid-switch path/to/script` : ignore lid close switch until the script exits
- `systemd-inhibit --what handle-power-key command` : ignore power button press while command is running
- `systemd-inhibit --who $USER --why reason --what operation command` : describe who and why created the inhibitor (default: the command and its arguments for `--who` and `Unknown reason` for `--why`)

## Categories
system, network

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-inhibit.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-inhibit
