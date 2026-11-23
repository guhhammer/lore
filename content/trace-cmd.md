# trace-cmd

Utility to interact with the Ftrace Linux kernel internal tracer. Some subcommands such as `list`, `record`, `report` have their own usage documentation.

## Usage
- `sudo trace-cmd stat` : display the status of tracing system
- `sudo trace-cmd list -t` : list available tracers
- `sudo trace-cmd start -p function|function_graph|preemptirqsoff|irqsoff|preemptoff|wakeup|...` : start tracing with a specific plugin
- `sudo trace-cmd show` : view the trace output
- `sudo trace-cmd stop` : stop the tracing but retain the buffers
- `sudo trace-cmd clear` : clear the trace buffers
- `sudo trace-cmd record` : record a trace
- `sudo trace-cmd report` : display the recorded trace

## Categories
users, media, documents

## Links
- More information: https://manned.org/trace-cmd
- tldr page: https://tldr.inbrowser.app/pages/linux/trace-cmd
