# cgcreate

Create cgroups, used to limit, measure, and control resources used by processes. `cgroups` types can be `memory`, `cpu`, `net_cls`, etc.

## Usage
- `cgcreate -g group_type:group_name` : create a new group
- `cgcreate -g group_type1,group_type2:group_name` : create a new group with multiple cgroup types
- `mkdir /sys/fs/cgroup/group_type/group_name/subgroup_name` : create a subgroup

## Categories
hardware, monitoring, processes

## Links
- More information: https://manned.org/cgcreate
- tldr page: https://tldr.inbrowser.app/pages/linux/cgcreate
