# prctl

Get or set the resource controls of running processes, tasks, and projects.

## Usage
- `prctl process_id` : examine process limits and permissions
- `prctl -P process_id` : examine process limits and permissions in machine parsable format
- `prctl -n process.max-file-descriptor process_id` : get a specific limit for a running process

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
processes, sunos

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
- tldr page: https://tldr.inbrowser.app/pages/sunos/prctl
