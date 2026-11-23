# jhsdb

Attach to a Java process or launch a postmortem debugger to analyze the core dump from a crashed Java Virtual Machine.

## Usage
- `jhsdb jstack --pid process_id` : print stack and locks information of a Java process
- `jhsdb clhsdb --core path/to/core_dump --exe path/to/jdk_or_bin_or_java` : open a core dump in interactive debug mode
- `jhsdb debugd --pid process_id --serverid optional_unique_id` : start a remote debug server
- `jhsdb clhsdb --pid process_id` : connect to a process in interactive debug mode

## Categories
processes, development

## Links
- More information: https://manned.org/jhsdb
- tldr page: https://tldr.inbrowser.app/pages/common/jhsdb
