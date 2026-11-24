# sc

Communicate with the Service Control Manager and services.

## Usage
- `sc query service_name` : show the status of a service (no service name will list all services)
- `sc start service_name` : start a service asynchronously
- `sc stop service_name` : stop a service asynchronously
- `sc create service_name binpath= path\to\service_binary_file` : create a service
- `sc delete service_name` : delete a service
- `sc config service_name type= service_type` : set the type of a service

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/sc
- Wikipedia: https://en.wikipedia.org/wiki/Sc_(spreadsheet_calculator)
