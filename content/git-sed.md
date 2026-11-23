# git sed

Replace patterns in git-controlled files using sed. Part of `git-extras`.

## Usage
- `git sed 'find_text' 'replace_text'` : replace the specified text in the current repository
- `git sed -c 'find_text' 'replace_text'` : replace the specified text and then commit the resulting changes with a standard commit message
- `git sed -f g 'find_text' 'replace_text'` : replace the specified text, using `regex`
- `git sed 'find_text' 'replace_text' -- path/to/directory` : replace a specific text in all files under a given directory

## Categories
version-control, text, files

## Links
- More information: https://manned.org/git-sed
- tldr page: https://tldr.inbrowser.app/pages/common/git-sed
