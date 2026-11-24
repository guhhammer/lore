# git send-email

Send a collection of patches as emails. Patches can be specified as files, directories, or a revision list.

## Usage
- `git send-email -1` : send the last commit in the current branch interactively
- `git send-email -1 commit` : send a given commit
- `git send-email -10` : send multiple (e.g. 10) commits in the current branch
- `git send-email -number_of_commits --compose` : send an introductory email message for the patch series
- `git send-email -number_of_commits --annotate` : review and edit the email message for each patch you're about to send

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-send-email
- tldr page: https://tldr.inbrowser.app/pages/common/git-send-email
- Wikipedia: https://en.wikipedia.org/wiki/Git
