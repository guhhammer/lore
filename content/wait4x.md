# wait4x

Wait for a port or service to enter a requested state, with support for TCP, HTTP, DNS, databases, and message queues. Some subcommands such as `tcp` and `http` have their own usage documentation.

## Usage
- `wait4x tcp localhost:8080` : wait for a TCP port to become available
- `wait4x http https://example.com/health --expect-status-code 200` : wait for an HTTP endpoint to return a specific status code
- `wait4x postgresql 'postgres://user:password@localhost:5432/mydb?sslmode=disable'` : wait for a PostgreSQL database to become ready
- `wait4x redis redis://localhost:6379` : wait for a Redis server to become available
- `wait4x tcp localhost:3306 --timeout 30s --interval 2s` : wait for a service with a custom timeout and check interval
- `wait4x tcp localhost:3306 localhost:6379 ...` : wait for multiple services in parallel
- `wait4x subcommand --help` : display help for a subcommand
- `wait4x version` : display version

## Categories
network, databases, system

## Links
- More information: https://github.com/wait4x/wait4x
- tldr page: https://tldr.inbrowser.app/pages/common/wait4x
