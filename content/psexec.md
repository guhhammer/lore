# psexec

Execute a command-line process on a remote machine. This is an advanced command and it might potentially be dangerous.

## Usage
- `psexec \\remote_host cmd` : execute a command using `cmd` in a remote shell
- `psexec \\remote_host -u user_name -p password` : execute a command on a remote host (pre-authenticated)
- `psexec \\remote_host cmd /c command -an ^>path\to\file.txt` : execute a command remotely and output the result to a file
- `psexec \\remote_host -d -i program_name` : execute a program to interact with users
- `psexec \\remote_host ipconfig /all` : display the IP configuration of the remote host

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, files, processes, windows

## Links
- More information: https://learn.microsoft.com/sysinternals/downloads/psexec
- tldr page: https://tldr.inbrowser.app/pages/windows/psexec
