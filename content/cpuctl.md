# cpuctl

Control and inspect system CPUs from command line. Note: At least one CPU in the system must always remain online.

## Usage
- `sudo cpuctl list` : display the current state and time of last state change for each CPU in the system
- `sudo cpuctl offline cpu_number1 cpu_number2 ...` : set the specified CPUs offline
- `sudo cpuctl online cpu_number1 cpu_number2 ...` : set the specified CPUs online

## Categories
hardware, shell, text

## Links
- More information: https://man.netbsd.org/cpuctl.8
- tldr page: https://tldr.inbrowser.app/pages/common/cpuctl
