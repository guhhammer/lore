# beanstalkd

A simple and generic work-queue server.

## Usage
- `beanstalkd` : start the server, listening on port 11300
- `beanstalkd -l ip_address -p port_number` : listen on a specific port and address
- `beanstalkd -b path/to/persistence_directory` : persist work queues by saving them to disk
- `beanstalkd -b path/to/persistence_directory -f 500` : sync to the persistence directory every 500 milliseconds

## Categories
network, files

## Links
- More information: https://manned.org/beanstalkd
- tldr page: https://tldr.inbrowser.app/pages/common/beanstalkd
