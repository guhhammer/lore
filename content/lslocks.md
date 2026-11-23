# lslocks

List local system locks.

## Usage
- `lslocks` : list all local system locks
- `lslocks -o PID,COMMAND,PATH,...` : list locks with defined column headers
- `lslocks -r -n` : list locks producing a raw output (no columns), and without column headers
- `lslocks -p process_id` : list locks by PID input
- `lslocks -J` : list locks with JSON output to `stdout`

## Categories
text

## Links
- More information: https://manned.org/lslocks
- tldr page: https://tldr.inbrowser.app/pages/linux/lslocks
