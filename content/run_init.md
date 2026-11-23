# run_init

Run init scripts in the proper SELinux context. Typically used to run system service scripts with correct SELinux domains.

## Usage
- `sudo run_init path/to/script` : run a script in the init script context
- `sudo run_init path/to/script start|stop|restart` : run a script with arguments
- `sudo run_init -t context_type path/to/script` : run a script and specify the init script context explicitly
- `sudo run_init -n path/to/script` : simulate running a script, displaying the context that would be used

## See also
- runcon
- semanage

## Categories
development, files, users

## Links
- More information: https://manned.org/run_init
- tldr page: https://tldr.inbrowser.app/pages/linux/run_init
