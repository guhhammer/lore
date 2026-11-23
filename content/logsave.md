# logsave

Save the output of a command in a logfile.

## Usage
- `logsave path/to/logfile command` : execute command with specified argument(s) and save its output to log file
- `logsave logfile -` : take input from `stdin` and save it in a log file
- `logsave -a logfile command` : append the output to a log file, instead of replacing its current contents
- `logsave -v logfile command` : show verbose output

## Categories
files, monitoring

## Links
- More information: https://manned.org/logsave
- tldr page: https://tldr.inbrowser.app/pages/linux/logsave
