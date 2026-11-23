# trace-cmd record

Capture kernel trace events.

## Usage
- `sudo trace-cmd record -p plugin` : record a trace with a specific plugin
- `sudo trace-cmd record -F executable` : record a trace of a specific executable
- `sudo trace-cmd record -g function` : record a trace of a specific function
- `sudo trace-cmd record -n function` : exclude a specific function from the trace
- `sudo trace-cmd record --max-graph-depth depth` : limit the function call graph depth
- `sudo trace-cmd record -P process_id` : record a trace from a specific process ID

## See also
- trace-cmd list
- trace-cmd report

## Categories
media, users, system

## Links
- More information: https://manned.org/trace-cmd-record
- tldr page: https://tldr.inbrowser.app/pages/linux/trace-cmd-record
