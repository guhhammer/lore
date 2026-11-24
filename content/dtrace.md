# dtrace

A simple interface to invoke the D language compiler, retrieve buffered trace, and print traced data from the DTrace kernel facility. Generic front-end to DTrace facility, requiring root privileges.

## Usage
- `dtrace -arch arch_name` : set target data model for a specific architecture
- `dtrace -a` : claim anonymous tracing state and display the traced data
- `dtrace -b 2g` : set principal trace buffer size. Supported units are `k`, `m`, `g`, or `t`
- `dtrace -s D_script` : compile the specified D Program source file
- `dtrace -c command` : run the specified command and exit upon its completion
- `dtrace -f function` : specify function name to trace or list (-l option). The corresponding argument can include any of the probe description forms like `provider:module:function`, `module:function`, or `function`
- `dtrace -p process_id` : grab the specified process ID, cache its symbol table, and exit upon completion
- `dtrace -a -b buffer_size -f function -p process_id` : combine different options for tracing function in a process

## Categories
development, network, system, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/dtrace.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/dtrace
- Wikipedia: https://en.wikipedia.org/wiki/DTrace
