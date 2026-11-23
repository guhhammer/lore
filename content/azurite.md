# azurite

Azure Storage API compatible server (emulator) in local environment.

## Usage
- `azurite -l path/to/directory` : use an existing location as workspace path
- `azurite -s` : disable access log displayed in console
- `azurite -d path/to/debug.log` : enable debug log by providing a file path as log destination
- `azurite --blobHost|--queueHost|--tableHost 0.0.0.0` : customize the listening address of Blob/Queue/Table service
- `azurite --blobPort|--queuePort|--tablePort 8888` : customize the listening port of Blob/Queue/Table service

## Categories
files, monitoring, cloud

## Links
- More information: https://www.npmjs.com/package/azurite
- tldr page: https://tldr.inbrowser.app/pages/common/azurite
