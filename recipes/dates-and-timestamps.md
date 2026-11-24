# dates and timestamps

Print dates in any format, turn epoch seconds into dates and back, use other time zones.

## Steps
- `date +%F` : today as 2026-11-23
- `date '+%Y-%m-%d %H:%M:%S'` : date and time
- `date +%s` : seconds since 1970 (epoch)
- `date -d @1700000000` : an epoch time as a date (GNU; macOS: date -r 1700000000)
- `date -d 'yesterday' +%F` : yesterday, next monday, 3 days ago… (GNU)
- `TZ=UTC date` : the time in another zone

## See also
- date

## Categories
shell text
