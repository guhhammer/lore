# taskkill

Terminate a process by its process ID or name.

## Usage
- `taskkill /pid process_id` : terminate a process by its ID
- `taskkill /im process_name` : terminate a process by its name
- `taskkill /pid process_id /f` : forcefully terminate a specified process
- `taskkill /im process_name /t` : terminate a process and its child processes
- `taskkill /pid process_id /s remote_name` : terminate a process on a remote machine
- `taskkill /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
processes, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/taskkill
