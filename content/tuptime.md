# tuptime

Report historical and statistical real time of the system, preserving it between restarts. Like `uptime`, but extended with startups count, graceful and bad shutdowns, uptime/downtime percentages, and averages since first boot.

## Usage
- `tuptime` : show system uptime history and statistics
- `tuptime --table` : show system lifetime as table
- `tuptime --list` : show system lifetime as list
- `tuptime --table --kernel --bootid` : show table with kernel version and boot ID
- `tuptime --table --since 1 --until 5` : report from startup range
- `tuptime --tsince -31557600` : limit the report from a given number of seconds ago (e.g. one year = 31557600)
- `tuptime --seconds` : output time in seconds and datetimes in epoch format
- `tuptime --csv` : report in CSV format

## Categories
monitoring, system, databases

## Links
- More information: https://github.com/rfmoz/tuptime
- tldr page: https://tldr.inbrowser.app/pages/common/tuptime
