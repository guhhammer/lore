# wine sc

Create, query, and control Windows services in a Wine prefix. A reimplementation of the Windows `sc` service control command.

## Usage
- `wine sc query service_name` : query the status of a service
- `wine sc create service_name binpath= C:\path\to\service.exe` : create a new service from an executable (note the space after `binpath=`)
- `wine sc start|stop service_name` : start or stop a service
- `wine sc description service_name "description_text"` : set the description of a service
- `wine sc delete service_name` : delete a service
- `wine sc /?` : display help

## Categories
system, databases

## Links
- More information: https://gitlab.winehq.org/wine/wine/-/wikis/Commands
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-sc
- Wikipedia: https://en.wikipedia.org/wiki/Wine_(software)
