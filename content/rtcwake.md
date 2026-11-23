# rtcwake

Enter a system sleep state until specified wakeup time relative to your BIOS clock.

## Usage
- `sudo rtcwake -m show -v` : show whether an alarm is set or not
- `sudo rtcwake -m mem -s 10` : suspend to RAM and wake up after 10 seconds
- `sudo rtcwake -m disk --date +15min` : suspend to disk (higher power saving) and wake up 15 minutes later
- `sudo rtcwake -m freeze --date YYYYMMDDhhmm` : freeze the system (more efficient than suspend-to-RAM but version 3.9 or newer of the Linux kernel is required) and wake up at a given date and time
- `sudo rtcwake -m disable` : disable a previously set alarm
- `sudo rtcwake -m on --date hh:ss` : simulate waking up the computer at a given time. (Press `<Ctrl c>` to abort)

## Categories
users, system, hardware

## Links
- More information: https://manned.org/rtcwake
- tldr page: https://tldr.inbrowser.app/pages/linux/rtcwake
