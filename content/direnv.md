# direnv

Shell extension to load and unload environment variables depending on the current directory.

## Usage
- `direnv allow .` : grant direnv permission to load the `.envrc` present in the current directory
- `direnv deny .` : revoke the authorization to load the `.envrc` present in the current directory
- `direnv edit .` : edit the `.envrc` file in the default text editor and reload the environment on exit
- `direnv reload` : trigger a reload of the environment
- `direnv status` : print some debug status information

## Categories
files, monitoring, shell

## Links
- More information: https://github.com/direnv/direnv/blob/master/man/direnv.1.md
- tldr page: https://tldr.inbrowser.app/pages/common/direnv
