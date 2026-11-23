# nodemon

Watch files and automatically restart a node application when changes are detected.

## Usage
- `nodemon path/to/file.js` : execute the specified file and watch a specific file for changes
- `rs` : [Interactive] Manually restart nodemon
- `nodemon -i path/to/file_or_directory` : ignore specific files
- `nodemon path/to/file.js arguments` : pass arguments to the node application
- `nodemon arguments path/to/file.js` : pass arguments to node itself if they're not nodemon arguments already (e.g. `--inspect`)
- `nodemon -x "command_to_run_script options" path/to/script` : run an arbitrary non-node script
- `nodemon -x "python options" path/to/file.py` : run a Python script

## Categories
files, development

## Links
- More information: https://github.com/remy/nodemon/tree/main/doc/cli
- tldr page: https://tldr.inbrowser.app/pages/common/nodemon
