# waitpid

Wait for the termination of arbitrary processes.

## Usage
- `waitpid process_id1 process_id2 ...` : sleep until all processes whose PIDs have been specified have exited
- `waitpid -t n process_id1 process_id2 ...` : sleep for at most `n` seconds
- `waitpid -e process_id1 process_id2 ...` : do not error if specified PIDs have already exited
- `waitpid -c n process_id1 process_id2 ...` : sleep until `n` of the specified processes have exited
- `waitpid -h` : display help

## See also
- wait

## Categories
processes

## Links
- More information: https://manned.org/waitpid
- tldr page: https://tldr.inbrowser.app/pages/linux/waitpid
