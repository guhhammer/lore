# mutagen

Real-time file synchronization and network forwarding tool.

## Usage
- `mutagen sync create --name=session_name /path/to/local_directory/ user@host:/path/to/remote_directory/` : start a synchronization session between a local directory and a remote host
- `mutagen sync create --name=session_name /path/to/local_directory/ docker://user@container_name/path/to/remote_directory/` : start a synchronization session between a local directory and a Docker container
- `mutagen sync terminate session_name` : stop a running session
- `mutagen project start` : start a project
- `mutagen project terminate` : stop a project
- `mutagen project list` : list running sessions for the current project

## Categories
files, network, development

## Links
- More information: https://mutagen.io/documentation/introduction/
- tldr page: https://tldr.inbrowser.app/pages/common/mutagen
