# disown

Allow sub-processes to live beyond the shell that they are attached to.

## Usage
- `disown` : disown the current job
- `disown %job_number` : disown a specific job (run `jobs` to find the job number)
- `disown -a` : disown all jobs (Bash only)
- `disown -h %job_number` : keep job (do not disown it), but mark it so that no future `SIGHUP` is received on shell exit (Bash only)

## See also
- jobs

## Categories
processes, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-disown
- tldr page: https://tldr.inbrowser.app/pages/common/disown
