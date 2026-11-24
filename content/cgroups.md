# cgroups

Linux kernel feature for limiting, measuring, and controlling resource usage by processes. Cgroups (control groups) however is not a command, but rather a collection of commands, see the relevant pages below.

## Usage
- `cgclassify -g cpu:student 1234` : move the process with a specific PID to the control group student in the CPU hierarchy
- `cgclassify 1234` : move the process with a specific PID to control groups based on the `/etc/cgrules.conf` configuration file
- `cgcreate -g group_type:group_name` : create a new group
- `cgcreate -g group_type1,group_type2:group_name` : create a new group with multiple cgroup types
- `cgexec -g controller:cgroup_name process_name` : execute a process in a given cgroup with given controller

## Categories
processes, users, system

## Links
- More information: https://www.kernel.org/doc/Documentation/cgroup-v2.txt
- tldr page: https://tldr.inbrowser.app/pages/linux/cgroups
- Wikipedia: https://en.wikipedia.org/wiki/Cgroups
