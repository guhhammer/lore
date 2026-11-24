# redis

Redis is an in-memory key-value store. redis-cli is the client for connecting, looking at keys and checking the server.

## Tasks
### Connect
- `redis-cli` : connect to the local server
- `redis-cli -h HOST -p 6379 -a PASSWORD` : connect to a remote server
- `redis-cli -u redis://USER:PASSWORD@HOST:6379/0` : connect with a URL
- `redis-cli ping` : check the server answers PONG
- `redis-cli -n 1` : use database number 1
### Keys and strings
- `SET NAME value` : store a value
- `GET NAME` : read it
- `SET NAME value EX 60` : store it and let it expire in 60 seconds
- `DEL NAME` : delete a key
- `EXISTS NAME` : 1 if the key exists
- `TTL NAME` : seconds left before it expires
- `INCR counter` : add one to a number
- `SCAN 0 MATCH user:* COUNT 100` : look through keys without blocking the server
### Other types
- `HSET user:1 name Ana age 30` : set fields of a hash
- `HGETALL user:1` : read a whole hash
- `LPUSH queue job1` : add to a list
- `LRANGE queue 0 -1` : read a whole list
- `SADD tags a b c` : add to a set
- `ZADD board 10 ana` : add to a sorted set with a score
### Check the server
- `INFO memory` : memory use
- `INFO keyspace` : keys per database
- `redis-cli --latency` : measure round trips
- `redis-cli monitor` : print every command as it arrives (slows the server)
- `CONFIG GET maxmemory` : read a setting
### Save and clean
- `BGSAVE` : write a snapshot to disk in the background
- `FLUSHDB` : delete every key of the current database (deletes data)
- `FLUSHALL` : delete every key of every database (deletes data)

## Notes
- Never use `KEYS *` on a busy server: it blocks it. Use `SCAN`.
- Redis listens on 127.0.0.1 by default; do not open it to the internet without a password and a firewall.
- Data lives in memory: without `BGSAVE` or the append-only file, a restart loses it.
- Keys with `user:1` style names are a convention, not a rule.

## See also
- mysql
- postgresql
- memcached

## Links
- Commands: https://redis.io/docs/latest/commands/
- redis-cli: https://redis.io/docs/latest/develop/tools/cli/

## Categories
database
