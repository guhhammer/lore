# cpulimit

A tool to throttle the CPU usage of other processes.

## Usage
- `cpulimit -p 1234 -l 25%` : limit an existing process with PID 1234 to only use 25% of the CPU
- `cpulimit -e program -l 25` : limit an existing program by its executable name
- `cpulimit -l 50 -- program argument1 argument2 ...` : launch a given program and limit it to only use 50% of the CPU
- `cpulimit -l 50 -b -- program` : launch a program, limit its CPU usage to 50% and run cpulimit in the background
- `cpulimit -l 50 -k -- program` : kill its process if the program's CPU usage goes over 50%
- `cpulimit -l 25 -m -- program` : throttle both it and its child processes so that none go about 25% CPU

## Categories
monitoring, hardware, processes

## Links
- More information: https://manned.org/cpulimit
- tldr page: https://tldr.inbrowser.app/pages/linux/cpulimit
