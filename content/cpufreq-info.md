# cpufreq-info

Show CPU frequency information.

## Usage
- `cpufreq-info` : show CPU frequency information for all CPUs
- `cpufreq-info -c cpu_number` : show CPU frequency information for the specified CPU
- `cpufreq-info -l` : show the allowed minimum and maximum CPU frequency
- `cpufreq-info -o` : show the current minimum and maximum CPU frequency and policy in table format
- `cpufreq-info -g` : show available CPU frequency policies
- `cpufreq-info -f -m` : show current CPU work frequency in a human-readable format, according to the cpufreq kernel module
- `sudo cpufreq-info -w -m` : show current CPU work frequency in a human-readable format, by reading it from hardware (only available to root)

## Categories
hardware

## Links
- More information: https://manned.org/cpufreq-info
- tldr page: https://tldr.inbrowser.app/pages/linux/cpufreq-info
