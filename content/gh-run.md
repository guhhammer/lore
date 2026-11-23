# gh run

View, run, and watch recent GitHub Actions workflow runs.

## Usage
- `gh run view` : interactively select a run to see information about the jobs
- `gh run view workflow_run_number` : display information about a specific run
- `gh run view -j job_number` : display information about the steps of a job
- `gh run view -j job_number --log` : display the log of a job
- `gh run view workflow_run_number --exit-status && echo "run pending or passed"` : check a specific workflow and exit with a non-zero status if the run failed
- `gh run watch` : interactively select an active run and wait until it's done
- `gh run watch workflow_run_number` : display the jobs for a run and wait until it's done
- `gh run rerun workflow_run_number` : re-run a specific workflow

## Categories
desktop, processes

## Links
- More information: https://cli.github.com/manual/gh_run
- tldr page: https://tldr.inbrowser.app/pages/common/gh-run
