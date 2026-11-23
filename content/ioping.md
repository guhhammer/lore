# ioping

Monitor I/O latency in real time.

## Usage
- `ioping .` : show disk I/O latency using the default values and the current directory
- `ioping -c 10 -s 1M /tmp` : measure latency on `/tmp` using 10 requests of 1 megabyte each
- `ioping -R /dev/sdX` : measure disk seek rate on `/dev/sdX`
- `ioping -RL /dev/sdX` : measure disk sequential speed on `/dev/sdX`

## Categories
disks, monitoring

## Links
- More information: https://github.com/koct9i/ioping
- tldr page: https://tldr.inbrowser.app/pages/common/ioping
