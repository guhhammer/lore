# doppler run

Run a command with Doppler secrets injected into the environment.

## Usage
- `doppler run --command command` : run a command
- `doppler run --command command1 && command2` : run multiple commands
- `doppler run path/to/command.sh` : run a script
- `doppler run -p project_name -c config_name -- command` : run command with specified project and config
- `doppler run --watch command` : automatically restart process when secrets change

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/doppler-run
