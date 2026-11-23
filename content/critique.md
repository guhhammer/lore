# critique

Review Git changes with syntax highlighting and word-level diffs.

## Usage
- `critique` : review unstaged changes in the working tree
- `critique --staged` : review only staged changes
- `critique commit` : review the changes introduced by a specific commit
- `critique branch_1 branch_2` : compare two branches
- `critique origin/branch HEAD` : review the local `HEAD` against its remote-tracking branch
- `critique --watch` : review changes, automatically refreshing when files change
- `critique --filter "src/**/*.ts"` : restrict the review to files matching a `glob` pattern
- `critique review --agent claude` : generate an AI code review using a specific agent

## Categories
version-control, text

## Links
- More information: https://github.com/remorses/critique#usage
- tldr page: https://tldr.inbrowser.app/pages/common/critique
