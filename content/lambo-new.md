# lambo new

A super-powered `laravel new` for Laravel and Valet.

## Usage
- `lambo new app_name` : create a new Laravel application
- `lambo new -p path/to/directory app_name` : install the application in a specific path
- `lambo new --auth app_name` : include authentication scaffolding
- `lambo new --vue|bootstrap|react app_name` : include a specific frontend
- `lambo new --node app_name` : install `npm` dependencies after the project has been created
- `lambo new -l app_name` : create a Valet site after the project has been created
- `lambo new --create-db --dbuser=user --dbpassword=password app_name` : create a new MySQL database with the same name as the project
- `lambo new -e "editor" app_name` : open a specific editor after the project has been created

## Categories
development, files, packages

## Links
- More information: https://github.com/tighten/lambo#lambo-commands
- tldr page: https://tldr.inbrowser.app/pages/common/lambo-new
