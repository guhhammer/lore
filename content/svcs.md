# svcs

List information about running services.

## Usage
- `svcs` : list all running services
- `svcs -vx` : list services that are not running
- `svcs apache` : list information about a service
- `svcs -L apache` : show location of service log file
- `tail $(svcs -L apache)` : display end of a service log file

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, sunos

## Links
- More information: https://www.unix.com/man-page/sunos/1/svcs
- tldr page: https://tldr.inbrowser.app/pages/sunos/svcs
