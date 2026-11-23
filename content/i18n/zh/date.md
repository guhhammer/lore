# date

设置或显示系统日期。

## Usage
- `date +%c` : 使用默认区域设置的格式显示当前日期
- `date -u +%Y-%m-%dT%H:%M:%SZ` : 以 UTC 和 ISO 8601 格式显示当前日期
- `date +%s` : 将当前日期显示为 unix 时间戳（自 1970-01-01 00:00:00 以来的秒数）
- `date -r 1473305798` : 使用默认格式显示特定日期（格式化指定 UNIX 时间戳）
- `date -v +1d -v -20m` : 使用默认格式显示相对于当前日期的日期

## Links
- More information: https://keith.github.io/xcode-man-pages/date.1.html
