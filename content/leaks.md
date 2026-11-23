# leaks

Search a process's memory for unreferenced malloc buffers.

## Usage
- `leaks process_name` : examine a running process for leaks by its name
- `leaks process_id` : examine a running process for leaks by its PID
- `leaks -atExit -- command` : launch a command and check for leaks when it exits
- `MallocStackLogging=YES leaks process_name` : enable backtraces for each leak by setting the `MallocStackLogging` environment variable
- `leaks -list process_name` : display results in the classic list format instead of a tree
- `leaks -groupByType process_name` : group leaked objects by type
- `leaks process_name -outputGraph path/to/file.memgraph` : save the memory state to a memory graph file for later analysis
- `leaks path/to/file.memgraph` : examine a previously saved memory graph file

## Categories
files, monitoring, processes, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/leaks.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/leaks
