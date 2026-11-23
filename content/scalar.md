# scalar

A tool for managing large Git repositories.

## Usage
- `scalar clone [--single-branch] [--branch <main-branch>] [--full-clone] [--[no-]src] <url> [<enlistment>]` : the general form, as the manual gives it
- `scalar list` : another form of the same command
- `scalar register [<enlistment>]` : another form of the same command
- `scalar unregister [<enlistment>]` : another form of the same command
- `scalar run ( all | config | commit-graph | fetch | loose-objects | pack-files ) [<enlistment>]` : another form of the same command
- `scalar -C <directory>` : before running the subcommand, change the working directory

## Categories
version-control
