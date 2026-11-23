# suspend

Suspend the execution of the current shell.

## Usage
- `bash <Enter> suspend` : suspend the current shell (useful for when you are in nested shells like `su`)
- `pkill -CONT bash` : continue from suspension if `suspend` was used in a non-nested shell (run this in a separate terminal)
- `suspend -f` : force suspension even if this would lock you out of the system

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-suspend
- tldr page: https://tldr.inbrowser.app/pages/common/suspend
