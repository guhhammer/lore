# distrobox-enter

Enter a Distrobox container. Default command executed is your `$SHELL`, but you can specify different shells or entire commands to execute. If used inside a script, an application, or a service, you can use the `--headless` mode to disable the tty and interactivity.

## Usage
- `distrobox-enter container_name` : enter a Distrobox container
- `distrobox-enter container_name -- sh -l` : enter a Distrobox container and run a command at login
- `distrobox-enter -n container_name -- uptime --pretty` : enter a Distrobox container without instantiating a tty

## See also
- distrobox

## Categories
shell, containers, development

## Links
- More information: https://distrobox.it/usage/distrobox-enter/
- tldr page: https://tldr.inbrowser.app/pages/linux/distrobox-enter
