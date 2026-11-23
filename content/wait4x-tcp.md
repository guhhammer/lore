# wait4x tcp

Wait for TCP port(s) to become available.

## Usage
- `wait4x tcp localhost:8080` : wait for a TCP port to become available
- `wait4x tcp localhost:3306 --timeout 60s` : wait for a port with a specific timeout
- `wait4x tcp localhost:8080 --invert-check` : wait for a port to become free (inverted check)
- `wait4x tcp localhost:3306 localhost:6379 ...` : wait for multiple ports in parallel
- `wait4x tcp localhost:3306 -- path/to/script.sh` : run a command after the port becomes available
- `wait4x tcp localhost:8080 --backoff-policy exponential --backoff-exponential-max-interval 30s` : wait with exponential backoff

## Categories
network

## Links
- More information: https://github.com/wait4x/wait4x
- tldr page: https://tldr.inbrowser.app/pages/common/wait4x-tcp
