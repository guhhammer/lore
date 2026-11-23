# callgrind_control

Observe and control programs being run by Callgrind.

## Usage
- `callgrind_control [options] [pid|program-name...]` : the general form, as the manual gives it
- `callgrind_control --version` : show version of callgrind_control
- `callgrind_control --instr=<on|off>` : switch instrumentation mode on or off
- `callgrind_control --vgdb-prefix=<prefix>` : specify the vgdb prefix to use by callgrind_control
