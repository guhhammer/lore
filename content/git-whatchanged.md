# git whatchanged

This command is an alias of `git log --raw --no-merges` (log with raw diffs for each commit). Note: This command is deprecated and will be removed in the future.

## Usage
- `log stream` : stream live system logs
- `log stream --process process_id` : stream logs sent to `syslog` from the process with a specific PID
- `log show --predicate "process == 'process_name'"` : show logs sent to syslog from a process with a specific name
- `sudo log collect --last 1h --output path/to/file.logarchive` : export all logs to disk for the past hour

## Categories
version-control, monitoring, processes

## Links
- More information: https://git-scm.com/docs/git-whatchanged
- tldr page: https://tldr.inbrowser.app/pages/common/git-whatchanged
- Wikipedia: https://en.wikipedia.org/wiki/Git
