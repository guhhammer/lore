# gnmic subscribe

Subscribe to a gnmic network device state updates.

## Usage
- `gnmic -a ip:port sub --path path` : subscribe to target state updates under the subtree of a specific path
- `gnmic -a ip:port sub --path path --sample-interval 30s` : subscribe to a target with a sample interval of 30s (default is 10s)
- `gnmic -a ip:port sub --path path --stream-mode on-change --heartbeat-interval 1m` : subscribe to a target with sample interval and updates only on change
- `gnmic -a ip:port sub --path path --mode once` : subscribe to a target for only one update
- `gnmic -a ip:port sub --path path -e json_ietf` : subscribe to a target and specify response encoding (json_ietf)

## Categories
files, network, hardware

## Links
- More information: https://gnmic.openconfig.net/cmd/subscribe/
- tldr page: https://tldr.inbrowser.app/pages/common/gnmic-subscribe
