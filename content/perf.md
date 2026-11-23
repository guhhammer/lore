# perf

Framework for Linux performance counter measurements.

## Usage
- `perf stat gcc hello.c` : display basic performance counter stats for a command
- `sudo perf top` : display system-wide real-time performance counter profile
- `sudo perf record command` : run a command and record its profile into `perf.data`
- `sudo perf record -p process_id` : record the profile of an existing process into `perf.data`
- `sudo perf report` : read `perf.data` (created by `perf record`) and display the profile

## Categories
media, monitoring, users

## Links
- More information: https://perfwiki.github.io/main/
- tldr page: https://tldr.inbrowser.app/pages/linux/perf
