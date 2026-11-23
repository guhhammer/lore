# envoy

A PHP-based task manager for Laravel remote servers.

## Usage
- `envoy init host_name` : initialize a configuration file
- `envoy run task_name` : run a task
- `envoy run --path path/to/directory task_name` : run a task from a specific project
- `envoy run --continue task_name` : run a task and continue on failure
- `envoy run --pretend task_name` : dump a task as a Bash script for inspection
- `envoy ssh server_name` : connect to the specified server via SSH

## Categories
files, network

## Links
- More information: https://laravel.com/docs/envoy
- tldr page: https://tldr.inbrowser.app/pages/common/envoy
