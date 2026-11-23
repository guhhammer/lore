# wsl

Manage the Windows Subsystem for Linux.

## Usage
- `wsl shell_command` : start a Linux shell (in the default distribution)
- `wsl -e command command_arguments` : run a Linux command without using a shell
- `wsl -d distribution shell_command` : specify a particular distribution
- `wsl -l` : list available distributions
- `wsl --export distribution path\to\distro_file.tar` : export a distribution to a `.tar` file
- `wsl --import distribution path\to\install_location path\to\distro_file.tar` : import a distribution from a `.tar` file
- `wsl --set-version distribution version` : change the version of wsl used for the specified distribution
- `wsl --shutdown` : shut down Windows Subsystem for Linux

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, archives, windows

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
- tldr page: https://tldr.inbrowser.app/pages/windows/wsl
