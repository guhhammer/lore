# gnunice

This command is an alias of GNU `nice`.

## Usage
- `nice` : print the current niceness value
- `nice nice` : increment the current niceness value by 10
- `nice -niceness_value command` : launch a program with lowered priority
- `sudo nice --niceness_value command` : launch a program with heightened priority
- `nice -n -20..19 command` : define the priority with an explicit option

## Categories
processes, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnunice
