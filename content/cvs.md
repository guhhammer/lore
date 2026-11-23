# cvs

Concurrent Versions System, a revision control system.

## Usage
- `cvs -d path/to/repository init` : create a new repository (requires the `$CVSROOT` environment variable to be set externally)
- `cvs import -m "message" project_name version vendor` : add a project to the repository
- `cvs checkout project_name` : check out a project
- `cvs diff path/to/file` : show changes made to files
- `cvs add path/to/file` : add a file
- `cvs commit -m "message" path/to/file` : commit a file
- `cvs update` : update the working directory from the remote repository

## Categories
files, packages, version-control

## Links
- More information: https://manned.org/cvs
- tldr page: https://tldr.inbrowser.app/pages/common/cvs
