# gnomon

Utility to annotate console logging statements with timestamps and find slow processes.

## Usage
- `npm test | gnomon` : use UNIX (or DOS) pipes to pipe `stdout` of any command through gnomon
- `npm test | gnomon -t elapsed-total` : show number of seconds since the start of the process
- `npm test | gnomon -t absolute` : show an absolute timestamp in UTC
- `npm test | gnomon -h 0.5` : use a high threshold of 0.5 seconds, exceeding which the timestamp will be colored bright red
- `npm test | gnomon -m 0.2` : use a medium threshold of 0.2 seconds, exceeding which the timestamp will be colored bright yellow

## Categories
development, processes

## Links
- More information: https://github.com/paypal/gnomon#options
- tldr page: https://tldr.inbrowser.app/pages/common/gnomon
