# fixfiles

Fix file SELinux security contexts.

## Usage
- `fixfiles -B` : record the current date in the `/.autorelabel` file when used with `onboot` (to speed up labeling later), or only restore files modified today when used with `restore`
- `fixfiles -F` : force reset of context to match `file_context` for customizable files
- `fixfiles -f` : clear `/tmp` directory without confirmation
- `fixfiles -R rpm_package1,rpm_package2 ...` : use the Rpm database to discover all files within specific packages and restore the file contexts
- `fixfiles -C PREVIOUS_FILECONTEXT` : run a diff on the `PREVIOUS_FILECONTEXT` file to the Currently installed one, and restore the context of all affected files
- `fixfiles -N YYYY-MM-DD HH:MM` : only act on files created after a specific date which will be passed to find `--newermt` command
- `fixfiles -M` : bind Mount filesystems before relabeling them, this allows fixing the context of files or directories that have been mounted over
- `fixfiles -v` : modify verbosity from progress to verbose and run `restorecon` with `-v` instead of `-p`

## Categories
files

## Links
- More information: https://manned.org/fixfiles
- tldr page: https://tldr.inbrowser.app/pages/linux/fixfiles
