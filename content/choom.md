# choom

Display and change the adjust out-of-memory killer score.

## Usage
- `choom -p process_id` : display the OOM-killer score of the process with a specific ID
- `choom -p process_id -n -1000..+1000` : change the adjust OOM-killer score of a specific process
- `choom -n -1000..+1000 command argument1 argument2 ...` : run a command with a specific adjust OOM-killer score

## Categories
desktop, monitoring

## Links
- More information: https://manned.org/choom
- tldr page: https://tldr.inbrowser.app/pages/linux/choom
