# az webapp

Manage Web Applications hosted in Azure Cloud Services. Part of `azure-cli` (also known as `az`).

## Usage
- `az webapp list-runtimes -os windows|linux` : list available runtimes for a web application
- `az webapp up -n name -l location -r runtime` : create a web application
- `az webapp list` : list all web applications
- `az webapp delete -n name -g resource_group` : delete a specific web application

## Categories
cloud, system

## Links
- More information: https://learn.microsoft.com/cli/azure/webapp
- tldr page: https://tldr.inbrowser.app/pages/common/az-webapp
