# crane mutate

Modify image labels and annotations. The container must be pushed to a registry, and the manifest is updated there.

## Usage
- `crane mutate -a/-l annotation/label` : set new annotations or labels
- `crane mutate --append/--cmd/--entrypoint/-e/--exposed-ports var1 var2 ...` : append a tarball, or set the command/entrypoint/environment variables/exposed ports of an image
- `crane mutate -o path/to/tarball` : write the resulting image to a new tarball
- `crane mutate --set-platform platform_name` : set the platform of the mutated image in the form `os/arch/variant:osversion,platform`
- `crane mutate -t tag_name` : apply a new tag reference to the mutated image
- `crane mutate -u username` : set a new user
- `crane mutate -w path/to/work_directory` : set a new working directory
- `crane mutate -h` : display help

## Categories
containers, media, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-mutate
