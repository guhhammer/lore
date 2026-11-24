# nextflow

Run computational pipelines. Mostly used for bioinformatics workflows.

## Usage
- `nextflow run main.nf -resume` : run a pipeline, use cached results from previous runs
- `nextflow run user/repo -revision release_tag` : run a specific release of a remote workflow from GitHub
- `nextflow run workflow -work-dir path/to/directory -with-report report.html` : run with a given work directory for intermediate files, save execution report
- `nextflow log` : show details of previous runs in current directory
- `nextflow clean -force run_name` : remove cache and intermediate files for a specific run
- `nextflow list` : list all downloaded projects
- `nextflow pull user/repo -hub bitbucket` : pull the latest version of a remote workflow from Bitbucket
- `nextflow self-update` : update Nextflow

## Categories
files

## Links
- More information: https://www.nextflow.io/docs/latest/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/nextflow
- Wikipedia: https://en.wikipedia.org/wiki/Nextflow
