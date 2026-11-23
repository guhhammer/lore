# gist

Upload code to <https://gist.github.com>.

## Usage
- `gist --login` : log in to gist on this computer
- `gist file.txt file2.txt` : create a gist from any number of text files
- `gist -p -d "A meaningful description" file.txt` : create a private gist with a description
- `echo "hello world" | gist` : read contents from `stdin` and create a gist from it
- `gist -l` : list your public and private gists
- `gist -l username` : list all public gists for any user
- `gist -u gist_id file.txt` : update a gist using the ID from URL

## Categories
network, files

## Links
- More information: https://github.com/defunkt/gist
- tldr page: https://tldr.inbrowser.app/pages/common/gist
