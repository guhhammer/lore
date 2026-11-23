# b4 prep

Prepare a new patch series for submission to a mailing list.

## Usage
- `b4 prep -n series_name -f fork_point` : start a new patch series branch forked from a specific point
- `b4 prep --edit-cover` : edit the cover letter for the current series
- `b4 prep -c` : automatically populate the cover letter with recipients from commit trailers and maintainer files
- `b4 prep --check` : run local checks (e.g., `checkpatch.pl`) on the series
- `b4 prep -e base_branch` : enroll an existing branch as a b4-prep-managed branch
- `b4 prep --cleanup branch_name` : archive and clean up an obsolete prep-managed branch

## Categories
version-control

## Links
- More information: https://b4.docs.kernel.org/en/latest/contributor/prep.html
- tldr page: https://tldr.inbrowser.app/pages/linux/b4-prep
