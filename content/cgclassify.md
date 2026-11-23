# cgclassify

Move running tasks to `cgroups`.

## Usage
- `cgclassify -g cpu:student 1234` : move the process with a specific PID to the control group student in the CPU hierarchy
- `cgclassify 1234` : move the process with a specific PID to control groups based on the `/etc/cgrules.conf` configuration file
- `cgclassify --sticky -g cpu:/student 1234` : move the process with a specific PID to the control group student in the CPU hierarchy. Note: The daemon of the service `cgred` does not change `cgroups` of the specific PID and its children (based on `/etc/cgrules.conf`)

## Categories
files, processes, hardware

## Links
- More information: https://manned.org/cgclassify
- tldr page: https://tldr.inbrowser.app/pages/linux/cgclassify
