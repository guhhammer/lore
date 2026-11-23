# redis-server

Persistent key-value database.

## Usage
- `redis-server` : start Redis server, using the default port (6379), and write logs to `stdout`
- `redis-server --daemonize yes` : start Redis server, using the default port, as a background process
- `redis-server --port port --daemonize yes` : start Redis server, using the specified port, as a background process
- `redis-server path/to/redis.conf` : start Redis server with a custom configuration file
- `redis-server --loglevel warning|notice|verbose|debug` : start Redis server with verbose logging

## Categories
databases, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/redis-server
