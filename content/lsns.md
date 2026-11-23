# lsns

List information about all namespaces or about the specified namespace.

## Usage
- `lsns` : list all namespaces
- `lsns -J` : list namespaces in JSON format
- `lsns -p process_id` : list namespaces associated with the specified process
- `lsns -t mnt|net|ipc|user|pid|uts|cgroup|time` : list the specified type of namespaces only
- `lsns -o NS,TYPE,PID,COMMAND` : list namespaces, only showing the namespace ID, type, PID, and command

## Categories
processes

## Links
- More information: https://manned.org/lsns
- tldr page: https://tldr.inbrowser.app/pages/linux/lsns
