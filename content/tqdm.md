# tqdm

Show progress over time of a command.

## Usage
- `seq 10000000 | tqdm | command` : show iterations per second and use `stdout` afterwards
- `seq 10000000 | tqdm --total 10000000 | command` : create a progress bar
- `zip -r path/to/archive.zip path/to/directory | tqdm --total $(find path/to/directory | wc -l) --unit files --null` : create an archive out of a directory and use the file count of that directory to create a progress bar
- `tar vzcf path/to/archive.tar.gz path/to/directory 2>&1 | tqdm --total $(find path/to/directory | wc -l) --unit files --null` : create an archive with `tar` and create a progress bar (system agnostic, GNU `tar` uses `stdout` while BSD `tar` uses `stderr`)

## Categories
files, archives

## Links
- More information: https://tqdm.github.io/docs/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/tqdm
