# wait-process

Wait for the processes to be stopped before accepting more input. Note: This command can only be used through PowerShell.

## Usage
- `Stop-Process -Id process_id; Wait-Process -Id process_id` : stop a process and wait
- `Wait-Process -Name process_name -Timeout 30` : wait for processes for a specified time

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
processes, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/wait-process
