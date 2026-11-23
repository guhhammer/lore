# scriptlive

Execute a typescript created by the `script` command in real-time.

## Usage
- `scriptlive path/to/timing_file path/to/typescript` : execute a typescript in real-time
- `scriptlive path/to/timing_file path/to/typescript --divisor 2` : execute a typescript at double the original speed
- `scriptlive --log-in path/to/stdin_log_file path/to/typescript` : execute a typescript created using `--log-in` option of `script`
- `scriptlive path/to/timing_file path/to/typescript --maxdelay 2` : execute a typescript waiting at most 2 seconds between each command

## See also
- script

## Categories
files, development

## Links
- More information: https://manned.org/scriptlive
- tldr page: https://tldr.inbrowser.app/pages/linux/scriptlive
