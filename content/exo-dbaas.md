# exo dbaas

Manage Exoscale DBaaS.

## Usage
- `exo dbaas type list` : list available Database Service types
- `exo dbaas type show database_service_type --plans` : list available plans for a Database Service type
- `exo dbaas create database_service_type database_service_type_plan database_service_name --database_service_type-ip-filter 1.2.3.4/32` : create a new Database Service (an IP filter must be specified in order to access the service)
- `exo dbaas show database_service_name --uri` : show the connection URI for a Database Service
- `exo dbaas update database_service_name --maintenance-dow day_of_the_week --maintenance-time HH:MM:SS` : set a specified maintenance time and day of the week for a Database Service
- `exo dbaas subcommand --help-database_service_type` : get help for a specific Database Service type

## Categories
system, databases

## Links
- More information: https://community.exoscale.com/product/dbaas/
- tldr page: https://tldr.inbrowser.app/pages/common/exo-dbaas
