# apptainer run

Run the default runscript of an Apptainer container. The runscript is defined in the `%runscript` section of the container's definition file.

## Usage
- `apptainer run path/to/image.sif` : run the default runscript of a container
- `apptainer run path/to/image.sif arg1 arg2 ...` : run with arguments passed to the runscript
- `apptainer run -B path/to/source:path/to/destination path/to/image.sif` : run with a bind mount from host to container
- `apptainer run --env variable=value path/to/image.sif` : run with environment variables
- `apptainer run -C path/to/image.sif` : run in fully isolated mode (contained filesystem, PID, IPC, and clean environment)
- `apptainer run --writable-tmpfs path/to/image.sif` : run with a writable temporary filesystem overlay
- `apptainer run --nv path/to/image.sif` : run with NVIDIA GPU support
- `apptainer run -h` : display help

## See also
- apptainer exec
- apptainer shell

## Categories
containers, files, media

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_run.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-run
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
