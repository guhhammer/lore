# git hash-object

Compute the unique hash key of content and optionally create an object with specified type.

## Usage
- `git hash-object path/to/file` : compute the object ID without storing it
- `git hash-object -w path/to/file` : compute the object ID and store it in the Git database
- `git hash-object -t blob|commit|tag|tree path/to/file` : compute the object ID specifying the object type
- `cat path/to/file | git hash-object --stdin` : compute the object ID from `stdin`

## Categories
security, version-control, files

## Links
- More information: https://git-scm.com/docs/git-hash-object
- tldr page: https://tldr.inbrowser.app/pages/common/git-hash-object
- Wikipedia: https://en.wikipedia.org/wiki/Git
