# gnuenv

This command is an alias of GNU `env`.

## Usage
- `env` : show the environment
- `env program` : run a program. Often used in scripts after the shebang (#!) for looking up the path to the program
- `env -i program` : clear the environment and run a program
- `env -u variable program` : remove variable from the environment and run a program
- `env variable=value program` : set a variable and run a program
- `env variable1=value variable2=value variable3=value ... program` : set one or more variables and run a program
- `env -a custom_name program` : run a program under a different name

## Categories
shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnuenv
