# nohup

Allow for a process to live when the terminal gets killed.

## Usage
- `nohup command argument1 argument2 ...` : run a process that can live beyond the terminal
- `nohup command argument1 argument2 ... &` : launch `nohup` in background mode
- `nohup path/to/script.sh &` : run a shell script that can live beyond the terminal
- `nohup command argument1 argument2 ... >path/to/output_file &` : run a process and write the output to a specific file

## Categories
processes

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/nohup
- Wikipedia: https://en.wikipedia.org/wiki/Nohup
