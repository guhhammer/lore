# apptainer test

Run the testscript defined in an Apptainer container image. The testscript is defined in the `%test` section of the container's definition file.

## Usage
- `apptainer test path/to/image.sif` : run the testscript of a container
- `apptainer test -B path/to/source:path/to/destination path/to/image.sif` : run the testscript with a bind mount from host to container
- `apptainer test -C path/to/image.sif` : run the testscript in fully isolated mode (contained filesystem, PID, IPC, and clean environment)
- `apptainer test --writable-tmpfs path/to/image.sif` : run the testscript with a writable temporary filesystem overlay
- `apptainer test --app app_name path/to/image.sif` : run the testscript for a specific app within a container
- `apptainer test -h` : display help

## Categories
containers, development, files

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_test.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-test
