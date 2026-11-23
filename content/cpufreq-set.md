# cpufreq-set

A tool to modify CPU frequency settings. The frequency value should range between the output of command `cpufreq-info -l`.

## Usage
- `sudo cpufreq-set -c 1 -g userspace` : set the CPU frequency policy of CPU 1 to "userspace"
- `sudo cpufreq-set -c 1 -d min_frequency` : set the current minimum CPU frequency of CPU 1
- `sudo cpufreq-set -c 1 -u max_frequency` : set the current maximum CPU frequency of CPU 1
- `sudo cpufreq-set -c 1 -f work_frequency` : set the current work frequency of CPU 1

## Categories
hardware, users

## Links
- More information: https://manned.org/cpufreq-set
- tldr page: https://tldr.inbrowser.app/pages/linux/cpufreq-set
