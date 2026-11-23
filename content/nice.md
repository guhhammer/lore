# nice

Execute a program with a custom scheduling priority (niceness). Niceness values range from -20 (the highest priority) to 19 (the lowest). Note: Some modern schedulers ignore niceness or limit its effects inside autogroups.

## Usage
- `nice` : print the current niceness value
- `nice nice` : increment the current niceness value by 10
- `nice -niceness_value command` : launch a program with lowered priority
- `sudo nice --niceness_value command` : launch a program with heightened priority
- `nice -n -20..19 command` : define the priority with an explicit option

## Categories
processes

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/nice-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/nice
