# git archive

Create an archive of files from a tree.

## Usage
- `git archive -v HEAD` : create a `.tar` archive from the contents of the current `HEAD` and print it to `stdout`
- `git archive -v --format zip HEAD` : use the Zip format and report progress verbosely
- `git archive -v -o path/to/file.zip HEAD` : output the Zip archive to a specific file
- `git archive -o path/to/file.tar branch_name` : create a `.tar` archive from the contents of the latest commit of a specific branch
- `git archive -o path/to/file.tar HEAD:path/to/directory` : use the contents of a specific directory
- `git archive -o path/to/file.tar --prefix path/to/prepend/ HEAD` : prepend a path to each file to archive it inside a specific directory

## Categories
archives, files, version-control

## Links
- More information: https://git-scm.com/docs/git-archive
- tldr page: https://tldr.inbrowser.app/pages/common/git-archive
