# uclampset

Manipulate the utilization clamping attributes of the system or a process.

## Usage
- `uclampset [options] [-m uclamp_min] [-M uclamp_max] _command argument` : the general form, as the manual gives it
- `uclampset [options] [-m uclamp_min] [-M uclamp_max] -p PID` : another form of the same command
- `uclampset -m` : set util_min value
- `uclampset -M` : set util_max value
- `uclampset -p` : operate on an existing PID and do not launch a new task
- `uclampset -s` : set or retrieve the system-wide utilization clamping attributes

## Categories
processes

## Links
- Documentation: https://kernel.org/doc/html/latest/scheduler/sched-util-clamp.html
