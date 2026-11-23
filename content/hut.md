# hut

A CLI tool for sourcehut.

## Usage
- `hut init` : initialize `hut`'s configuration file (this will prompt for an OAuth2 access token, which is required to use `hut`)
- `hut git|hg list` : list Git/Mercurial repositories
- `hut git|hg create name` : create a public Git/Mercurial repository
- `hut builds list` : list jobs on <https://builds.sr.ht>
- `hut builds show job_id` : show the status of a job
- `hut ssh job_id` : SSH into a job container

## Categories
version-control, network, processes

## Links
- More information: https://manned.org/hut
- tldr page: https://tldr.inbrowser.app/pages/common/hut
