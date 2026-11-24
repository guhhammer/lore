# ip stats

Manage and show interface statistics.

## Usage
- `ip st` : show all interface statistics across all network devices
- `ip st show dev network_interface` : show statistics for a specific network interface
- `ip st show group link` : show link-layer statistics (same as `ip -statistics link show`)
- `ip st show group offload` : show hardware offload statistics for all devices
- `ip st show dev network_interface group offload` : show offload statistics for a specific interface
- `ip st show dev network_interface group offload subgroup l3_stats|cpu_hit|hw_stats_info` : show a specific offload subgroup
- `ip st show group afstats subgroup mpls` : show address-family specific statistics (e.g. MPLS)
- `ip st set dev network_interface l3_stats on` : enable Layer 3 hardware statistics collection on a device

## Categories
monitoring, network, users

## Links
- More information: https://manned.org/ip-stats
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-stats
- Wikipedia: https://en.wikipedia.org/wiki/Iproute2
