# tc

Show/manipulate traffic control settings.

## Usage
- `sudo tc q a dev eth0 root netem delay delay_in_millisecondsms` : add constant network delay to outbound packages
- `sudo tc q a dev eth0 root netem delay mean_delay_msms delay_std_msms` : add normal distributed network delay to outbound packages
- `sudo tc q a dev eth0 root netem corruption|loss|duplication effect_percentage%` : add package corruption/loss/duplication to a portion of packages
- `sudo tc q a dev eth0 root tbf rate max_bandwidth_mbmbit burst max_burst_rate_kbkbit latency max_latency_before_drop_msms` : limit bandwidth, burst rate, and max latency
- `tc q s dev eth0` : show active traffic control policies
- `sudo tc q d dev eth0` : delete all traffic control rules
- `sudo tc q c dev eth0 root netem policy policy_parameters` : change traffic control rule

## Categories
users, network, packages

## Links
- More information: https://manned.org/tc
- tldr page: https://tldr.inbrowser.app/pages/linux/tc
