# timeout

Run a command with a time limit.

## Usage
- `timeout 3s sleep 10` : run `sleep 10` and terminate it after 3 seconds
- `timeout -s INT|HUP|KILL|... 5s sleep 10` : send a signal to the command after the time limit expires (`TERM` by default, `kill -l` to list all signals)
- `timeout -v 0.5s|1m|1h|1d|... command` : send verbose output to `stderr` showing signal sent upon timeout
- `timeout -p 1s|1m|1h|1d|... command` : preserve the exit status of the command regardless of timing out
- `timeout -k 5m 30s command` : send a forceful `KILL` signal after certain duration if the command ignores initial signal upon timeout

## Categories
processes

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/timeout
