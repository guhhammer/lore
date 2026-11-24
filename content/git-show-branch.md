# git show-branch

Show branches and their commits.

## Usage
- `git show-branch branch_name|ref|commit` : show a summary of the latest commit on a branch
- `git show-branch branch_name1|ref1|commit1 branch_name2|ref2|commit2 ...` : compare commits in the history of multiple commits or branches
- `git show-branch -r` : compare all remote tracking branches
- `git show-branch -a` : compare both local and remote tracking branches
- `git show-branch -a --list` : list the latest commits in all branches
- `git show-branch --current commit|branch_name|ref` : compare a given branch with the current branch
- `git show-branch --sha1-name --current current|branch_name|ref` : display the commit name instead of the relative name
- `git show-branch --more 5 branch_name1|ref1|commit1 branch_name2|ref2|commit2 ...` : keep going a given number of commits past the common ancestor

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-show-branch
- tldr page: https://tldr.inbrowser.app/pages/common/git-show-branch
- Wikipedia: https://en.wikipedia.org/wiki/Git
