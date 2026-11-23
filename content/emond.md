# emond

Event Monitor service that accepts events from various services, runs them through a simple rules engine, and takes action. The actions can run commands, send email, or SMS messages.

## Usage
- `emond` : start the daemon
- `emond -r path/to/file_or_directory` : specify rules for emond to process by giving a path to a file or directory
- `emond -c path/to/config_file` : use a specific configuration file

## Categories
system, files, monitoring, macos

## Links
- More information: https://manpagez.com/man/8/emond/
- tldr page: https://tldr.inbrowser.app/pages/osx/emond
