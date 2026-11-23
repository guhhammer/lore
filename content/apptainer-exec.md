# apptainer exec

Execute a command within an Apptainer container.

## Usage
- `apptainer exec path/to/image.sif command` : execute a command inside a container
- `apptainer exec path/to/image.sif command arg1 arg2 ...` : execute a command with arguments
- `apptainer exec -B path/to/source:path/to/destination path/to/image.sif command` : execute a command with a bind mount from host to container
- `apptainer exec --env variable=value path/to/image.sif command` : execute a command with environment variables
- `apptainer exec -C path/to/image.sif command` : execute a command in fully isolated mode (contained filesystem, PID, IPC, and clean environment)
- `apptainer exec --writable-tmpfs path/to/image.sif command` : execute a command with a writable temporary filesystem overlay
- `apptainer exec --nv path/to/image.sif command` : execute a command with NVIDIA GPU support
- `apptainer exec -h` : display help

## See also
- apptainer run
- apptainer shell

## Categories
containers, files, media

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_exec.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-exec
