# wait

Wait for a process to complete before proceeding.

## Usage
- `wait process_id` : wait for a process to finish given its process ID and return its exit status
- `wait` : wait for all processes known to the invoking shell to finish
- `wait -n` : wait for one of the processes to finish
- `wait -n -p variable` : wait for one of the processes to finish and store its PID in a variable
- `wait %job_number` : wait for a job to finish (run `jobs` to find the job number)
- `wait --help` : display help

## See also
- ps
- waitpid

## Categories
processes, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-wait
- tldr page: https://tldr.inbrowser.app/pages/common/wait
