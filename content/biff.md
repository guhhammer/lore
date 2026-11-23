# biff

A simple utility for doing datetime arithmetic, parsing, and formatting.

## Usage
- `biff time fmt -f rfc3339 now` : print the current time in a format of your choosing
- `biff time fmt -f '%c' now -1d 'next sat' 'last monday' '9pm last mon'` : print multiple relative times in one command
- `biff time in Asia/Bangkok now | biff time round -i 15 -s minute` : print the current time in another time zone and round it the nearest 15 minute increment
- `TZ='Japan' biff time in America/New_York 02:30` : convert a time between two different time zone
- `biff time add -1d|1d|1w|-1m|1y|... now` : print a past or future time relative to current time
- `biff time add '1 week, 12 hours ago' now` : add a complex duration to the current time
- `biff span since 2025-01-20T12:00 -l year` : find the duration since a date in the past and round it to the desired precision
- `biff tag lines /tmp/access.log | biff time in system | biff time fmt -f '%c' | head -n 3 | biff untag -s` : find timestamps in a log file and reformat them into your local time in place

## Categories
documents

## Links
- More information: https://github.com/burntsushi/biff
- tldr page: https://tldr.inbrowser.app/pages/common/biff
