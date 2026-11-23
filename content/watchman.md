# watchman

A service that watches files, and triggers actions when changes occur.

## Usage
- `watchman watch-project path/to/directory` : infer the root directory of the project containing the specified directory, and watch its files and sub-folders for changes
- `watchman -- trigger path/to/watched_directory trigger_name 'pattern' -- command` : add a trigger to run a command when files with a specified filename pattern in a watched directory change
- `watchman watch-list` : list all watched directories
- `watchman watch-del path/to/watched_directory` : delete a watch on a directory and its associated triggers
- `watchman watch-del-all` : delete all watched directories and triggers
- `watchman trigger-list path/to/watched_directory` : list all triggers on a watched directory
- `watchman trigger-del path/to/watched_directory trigger_name` : delete a trigger from a watched directory
- `watchman shutdown-server` : temporarily stop `watchman`, until the next time you call a `watchman` command

## Categories
files, system

## Links
- More information: https://facebook.github.io/watchman/docs/cli-options
- tldr page: https://tldr.inbrowser.app/pages/common/watchman
