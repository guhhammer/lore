# wmic

Interactive shell for detailed information about running processes.

## Usage
- `wmic alias where_clause verb_clause` : run a command using the fundamental grammar
- `wmic process list brief` : show brief details about the currently running processes
- `wmic process list full` : show full details about the currently running processes
- `wmic process get name,process_id,parent_process_id` : access specific fields such as process name, process ID, and parent process ID
- `wmic process where name="example.exe" list full` : display information about a specific process
- `wmic process where processid=process_id get name,commandline` : display specific fields for a specific process
- `wmic process process_id delete` : kill a process

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
processes, shell, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/wmic
