# jcmd

Sends diagnostic commands to a running Java process.

## Usage
- `jcmd` : list running Java processes
- `jcmd 1234 VM.version` : show the JVM version of a process
- `jcmd 1234 Thread.print` : print the thread stacks of a process
- `jcmd 1234 GC.heap_info` : show the heap usage
- `jcmd 1234 help` : list the commands that process accepts

## Categories
processes
