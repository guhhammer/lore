# gnudate

This command is an alias of GNU `date`.

## Usage
- `date +%c` : display the current date using the default locale's format
- `date -u +%Y-%m-%dT%H:%M:%SZ` : display the current date in UTC, using the ISO 8601 format
- `date +%s` : display the current date as a Unix timestamp (seconds since the Unix epoch)
- `date -d @1473305798` : convert a date specified as a Unix timestamp to the default format
- `date -d "2018-09-01 00:00" +%s -u` : convert a given date to the Unix timestamp format
- `date --rfc-3339 date|seconds|ns` : display the current date using the RFC-3339 format (`YYYY-MM-DD hh:mm:ss+TZ`) and specify its precision
- `sudo date 093023592021.59` : set the current date using the format `MMDDhhmmYYYY.ss` (`YYYY` and `.ss` are optional)
- `date +%V` : display the current ISO week number

## Categories
desktop, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnudate
