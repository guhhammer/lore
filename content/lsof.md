# lsof

List open files and the corresponding processes. Note: Root privileges are required to list files opened by others.

## Usage
- `lsof path/to/file` : find the processes that have a given file open
- `lsof -i :port` : find the process that opened a local internet port
- `lsof -t path/to/file` : only output the process ID (PID)
- `lsof -u username` : list files opened by the given user
- `lsof -c process_or_command_name` : list files opened by the given command or process
- `lsof -p process_id` : list files opened by a specific process, given its PID
- `lsof +D path/to/directory` : list open files in a directory
- `lsof -i6TCP:port -sTCP:LISTEN -n -P` : find the process that is listening on a local IPv6 TCP port and don't convert network or port numbers

## Categories
processes, files, network

## Links
- More information: https://manned.org/lsof
- tldr page: https://tldr.inbrowser.app/pages/common/lsof
