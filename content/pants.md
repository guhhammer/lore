# pants

Fast, scalable, user-friendly, open-source build and developer workflow tool.

## Usage
- `pants list ::` : list all targets
- `pants test ::` : run all tests
- `pants --changed-since=HEAD fix fmt lint` : fix, format, and lint only uncommitted files
- `pants --changed-since=HEAD --changed-dependents=transitive check` : typecheck only uncommitted files and their dependents
- `pants package path/to/directory:target-name` : create a distributable package for the specified target
- `pants tailor ::` : auto-generate BUILD file targets for new source files
- `pants help` : display help

## Categories
development, files, users

## Links
- More information: https://www.pantsbuild.org/stable/docs/using-pants/command-line-help
- tldr page: https://tldr.inbrowser.app/pages/common/pants
