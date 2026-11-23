# dockerd

A persistent process to start and manage Docker containers.

## Usage
- `dockerd` : run Docker daemon
- `dockerd -H unix://path/to/tmp.sock -H tcp://ip_address` : run Docker daemon and configure it to listen to specific sockets (UNIX and TCP)
- `dockerd -p path/to/pid_file` : run with specific daemon PID file
- `dockerd -D` : run in debug mode
- `dockerd -l debug|info|warn|error|fatal` : run and set a specific log level

## Categories
containers, processes, files

## Links
- More information: https://docs.docker.com/reference/cli/dockerd/
- tldr page: https://tldr.inbrowser.app/pages/linux/dockerd
