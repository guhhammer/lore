# apptainer shell

Start an interactive shell within an Apptainer container.

## Usage
- `apptainer shell path/to/image.sif` : start an interactive shell inside a container
- `apptainer shell -B path/to/source:path/to/destination path/to/image.sif` : start a shell with a bind mount from host to container
- `apptainer shell --env variable=value path/to/image.sif` : start a shell with environment variables
- `apptainer shell -C path/to/image.sif` : start a shell in fully isolated mode (contained filesystem, PID, IPC, and clean environment)
- `apptainer shell --writable-tmpfs path/to/image.sif` : start a shell with a writable temporary filesystem overlay
- `apptainer shell --nv path/to/image.sif` : start a shell with NVIDIA GPU support
- `apptainer shell -s path/to/shell path/to/image.sif` : start a shell using a specific shell program
- `apptainer shell -h` : display help

## See also
- apptainer exec
- apptainer run

## Categories
shell, containers, files

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_shell.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-shell
